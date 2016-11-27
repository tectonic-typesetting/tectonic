/*------------------------------------------------------------------------
Copyright (C) 2002-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: Engine.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:


Changes:

	2008-01-23  jk  revised endian-ness stuff to allow Universal build
	2006-06-02	jk	added support for extended string rules (>255 per initial char)
	24-May-2005		change from Ulrik to work around MS VC++ 6 issues
	21-May-2005		changes based on Ulrik Petersen's patch for MS VC++ 6

-------------------------------------------------------------------------*/

#ifndef __Compiler_H__
#define __Compiler_H__

#ifdef HAVE_CONFIG_H
#	include "config.h"	/* a Unix-ish setup where we have config.h available */
#endif

#if	(defined(__dest_os) && (__dest_os == __win32_os)) || defined(WIN32)	/* Windows target: little-endian */
#	undef WORDS_BIGENDIAN
#endif

#ifdef __APPLE__
#include <TargetConditionals.h>
#endif

#if defined(TARGET_RT_BIG_ENDIAN)	/* the CodeWarrior prefix files or Apple TargetConditionals.h sets this */
#	if TARGET_RT_BIG_ENDIAN
#		undef WORDS_BIGENDIAN
#		define WORDS_BIGENDIAN 1
#	else
#		undef WORDS_BIGENDIAN
#	endif
#endif

#include "TECkit_Format.h"
#include "TECkit_Compiler.h"

#include "TECkit_Engine.h"

#ifndef __MWERKS__
#	include "ulong_chartraits.h"
#endif

#include <string>
#include <vector>
#include <map>

using namespace std;

class Compiler
{
public:
					Compiler(const char* txt, UInt32 len, char inForm, bool cmp, bool genXML, TECkit_ErrorFn errFunc, void* userData);
					~Compiler();
	
	void			GetCompiledTable(Byte*& table, UInt32& len) const;
	void			DetachCompiledTable();

	enum { kInvalidRuleOffset = 0xffffffffUL };

protected:
	typedef enum {
		// general token types recognized by the compiler
		tok_Newline = 256,
		tok_Map,
		tok_Ellipsis,
		tok_Number,
		tok_USV,
		tok_Identifier,
		tok_String,
		tok_Unknown,
		// then we have the TECkit language keywords:
		tok_Name,
		tok_Flags,
		tok_FlagValue,
		tok_Pass,
		tok_PassType,
		tok_Class,
		tok_Default,
		tok_Define
	} tokenType;

	Byte*		compiledTable;
	UInt32		compiledSize;

	TECkit_ErrorFn	errorFunction;
	void*			errFuncUserData;

	typedef basic_string<UInt32>	string32;

	struct Token {
		tokenType	type;
		UInt32		val;
		const char*	str;
		string32	strval;
	};
	
	struct Keyword {
		const char	*keyword;
		tokenType	token;
		UInt32		refCon;
	};
	static Keyword	keywords[];

	const unsigned char*	textEnd;
	const unsigned char*	textPtr;
	
	char		idBuffer[256];
	
	// used by the front end parser
	UInt32		ungotten;
	Token		tok;
	const unsigned char*	tokStart;
	UInt32		errorCount;
	UInt32		lineNumber;
	char		inputForm;
	bool		errorState;
	bool		generateXML;
	bool		usedExtStringRules;

	// used in compiling passes
	enum {
		notInRule,
		inLHSString,
		inLHSPreContext,
		inLHSPostContext,
		inRHSString,
		inRHSPreContext,
		inRHSPostContext
	}			ruleState;
	char		ruleType;

	struct Item {
		UInt8	type;	// 0: literal; kMatchElem_Type_XXXX; 0xff: copy
		UInt8	negate;
		UInt8	repeatMin;
		UInt8	repeatMax;
		UInt32	val;	// class index or literal value
		UInt8	start;	// OR/EGroup: index of BGroup
		UInt8	next;	// BGroup/OR: index of next OR/EGroup
		UInt8	after;	// BGroup: index of EGroup + 1
		UInt8	index;	// Class/Copy: index of corresponding item in match
		string	tag;
	};
	
	struct Rule {
						Rule(
							const vector<Item>&	mat,
							const vector<Item>&	pre,
							const vector<Item>&	post,
							const vector<Item>&	rep,
							UInt32				line
							)	: matchStr(mat)
								, preContext(pre)
								, postContext(post)
								, replaceStr(rep)
								, lineNumber(line)
								, offset(kInvalidRuleOffset)
								, sortKey(0)
							{ }
		vector<Item>	matchStr;
		vector<Item>	preContext;
		vector<Item>	postContext;
		vector<Item>	replaceStr;
		UInt32			lineNumber;
		UInt32			offset;	// offset of the packed form in the StringRuleData block
		UInt16			sortKey;
		UInt16			reserved;
	};
	
	struct CurrRule {
		void			clear();
		void			setLineNo(UInt32 lineNo);
		UInt32			startingLine;
		vector<Item>	lhsString;
		vector<Item>	lhsPreContext;
		vector<Item>	lhsPostContext;
		vector<Item>	rhsString;
		vector<Item>	rhsPreContext;
		vector<Item>	rhsPostContext;
	};
	
	CurrRule			currentRule;	// the current rule being parsed
	
	UInt32				classLine;

	typedef	vector<UInt32>	Class;

	struct MatClass {
						MatClass(UInt32 m)
							: membersClass(m)
								{ }
		UInt32			membersClass;
	};
	struct RepClass {
						RepClass(UInt32 m, UInt32 s)
							: membersClass(m)
							, sortLikeClass(s)
								{ }
		UInt32			membersClass;
		UInt32			sortLikeClass;
	};
	
	struct Pass {
		void				clear();
		void				setLineNo(UInt32 lineNo);
		UInt32				startingLine;
		vector<Rule>		fwdRules;
		vector<Rule>		revRules;
		vector<string>		xmlRules;
		map<string,string>	xmlContexts;

		map<string,UInt32>	byteClassNames;		// map name to byteClassMembers index
		map<string,UInt32>	uniClassNames;

		vector<Class>		byteClassMembers;	// the actual members of each byte class
		vector<Class>		uniClassMembers;
		vector<UInt32>		byteClassLines;
		vector<UInt32>		uniClassLines;

		UInt32				passType;
		UInt32				uniDefault;
		UInt8				byteDefault;
		bool				supplementaryChars;
	};
	
	Pass				currentPass;	// the current pass being built

	struct BuildVars {
		void				clear();
		string				planeMap;
		vector<string>		pageMaps;
		vector< vector<UInt16> >	charMaps;
		UInt8				maxMatch;
		UInt8				maxPre;
		UInt8				maxPost;
		UInt8				maxOutput;
	};
	
	BuildVars			buildVars;		// variables used during pass compilation
	
	vector<string>		fwdTables;		// binary forms of compiled tables
	vector<string>		revTables;
	
	UInt32				lhsFlags;
	UInt32				rhsFlags;
	
	map<UInt16,string>	names;			// map name IDs to name texts (NB: utf8)

	typedef vector<Token>		tokListT;
	tokListT::const_iterator	defIter;
	tokListT::const_iterator	defEnd;
	map<string,tokListT>		defines;

	string			xmlRepresentation;

	UInt32			getChar(void);
	void			ungetChar(UInt32 c);
	
	void			SkipSpaces(void);
	tokenType		IDlookup(const char* str, UInt32 len);
	bool			GetNextToken();
	bool			ExpectToken(tokenType type, const char* errMsg);
	bool			ExpectToken(char c, const char* errMsg)
						{ return ExpectToken((tokenType)c, errMsg); }
	void			Error(const char* errMsg, const char* s = 0, UInt32 line = 0xffffffff);
	void			StartDefaultPass();
	void			AppendLiteral(UInt32 val, bool negate = false);
	void			AppendUSV(UInt32 val, bool negate = false);
	void			AppendSpecial(UInt8 type, bool negate = false);
	void			AppendClass(const string& className, bool negate = false);
	void			AppendToRule(const Item& item);
	bool			tagExists(bool rhs, const string& tag);
	void			AssignTag(const string& tag);
	void			SetMinMax(int repeatMin, int repeatMax);
	void			FinishPass();
	string			asUTF8(const string32 s);
	void			ReadNameString(UInt16 nameID);
	
	UInt32			charLimit();
	static int		ruleKeyComp(const Rule& a, const Rule& b);
	int				findTag(const string& tag, const vector<Item>& str);
	void			associateItems(vector<Rule>& rules, bool fromUni, bool toUni);
	void			setGroupPointers(vector<Item>::iterator b, vector<Item>::iterator e, int startIndex, bool isReversed = false);
	void			setGroupPointers(vector<Rule>& rules);
	void			sortRules(vector<Rule>& rules);
	int				calcMaxLen(vector<Item>::iterator b, vector<Item>::iterator e);
	int				calcMaxOutLen(Rule& rule);
	bool			findInitialItems(const Rule& rule, vector<Item>::const_iterator b, vector<Item>::const_iterator e,
										vector<Item>& initialItems);
	void			findInitialItems(const Rule& rule, vector<Item>& initialItems);
	void			addToCharMap(UInt32 ch, UInt16 index);
	void			buildTable(vector<Rule>& rules, bool fromUni, bool toUni, string& table);
	long			classIndex(UInt32 charCode, const Class& classMembers);
	long			uniClassIndex(UInt32 charCode, UInt32 classIndex);
	long			byteClassIndex(UInt8 charCode, UInt32 classIndex);
	bool			isSingleCharRule(const Rule& rule);
	void			appendMatchElem(string& packedRule, Item& item, int index,
									vector<MatClass>& matchClasses);
	void			appendReplaceElem(string& packedRule, Item& item,
									vector<Item>& matchStr, vector<RepClass>& repClasses);
	void			appendToTable(string& s, const char* ptr, UInt32 len);
	template <class T>
		void		appendToTable(string& table, T x) {
#ifdef WORDS_BIGENDIAN
			const char*	xp = (const char*)&x;
			table.append(xp, sizeof(x));
#else
			/* split into separate statements to work around VC++6 problems */
 			const char*	xp = (const char*)&x;
 			xp = xp + sizeof(T);
 			for (unsigned int i = 0; i < sizeof(T); ++i) {
				xp = xp - 1;
				table.append(1, *xp);
 			}
#endif
	}

	vector<Item>	reverseContext(const vector<Item>& ctx);
	void			align(string& table, int alignment);
	
	void			xmlOut(const char* s);
	void			xmlOut(const string& s);
	void			xmlOut(char c);
	string			xmlString(vector<Item>::const_iterator b, vector<Item>::const_iterator e, bool isUnicode);
	string			getContextID(const vector<Item>& ctx, bool isUnicode);
};

extern "C" {
	struct CharName {
		unsigned int	usv;
		const char*		name;
	};
	extern CharName	gUnicodeNames[];
}

#endif	/* __Compiler_H__ */
