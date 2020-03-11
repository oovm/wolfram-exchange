Needs["JLink`"];
InstallJava[];
LoadJavaClass["java.awt.Toolkit", AllowShortContext -> False];

CopyAsUTF8[s_String] := JavaBlock[java`awt`Toolkit`getDefaultToolkit[]@getSystemClipboard[]@setContents[#, #]&@JavaNew["java.awt.datatransfer.StringSelection", s]];
systems = Names["System`*"];
TemplateApply[
	"pub const SYSTEM_SYMBOLS: [&str; `2`] = [\"`1`\"]",
	{StringRiffle[systems, "\",\""], Length@systems}
]
% // CopyAsUTF8;