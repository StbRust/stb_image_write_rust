using Hebron;
using Hebron.Rust;
using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Linq;

namespace StbSharp.StbImageWrite.Generator
{
	class Program
	{
		private static string Write(Dictionary<string, string> input)
		{
			var keys = (from string k in input.Keys orderby k select k).ToArray();

			var result = string.Empty;
			foreach (var key in keys)
			{
				var value = input[key];

				result += value;
				result += Environment.NewLine;
			}

			return result;
		}

		static void Process()
		{
			var parameters = new RustConversionParameters
			{
				InputPath = @"stb_image_write.h",
				Defines = new[]
				{
					"STBI_WRITE_NO_STDIO",
					"STB_IMAGE_WRITE_IMPLEMENTATION",
					"STB_IMAGE_WRITE_STATIC"
				},
				SkipStructs = new string[]
				{
				},
				SkipGlobalVariables = new string[]
				{
				},
				SkipFunctions = new string[]
				{
				}
			};

/*			var textResult = TextCodeConverter.Convert(parameters.InputPath, parameters.Defines);
			File.WriteAllText(@"..\..\..\..\..\..\StbImageWrite.txt", textResult);*/

			var result = RustCodeConverter.Convert(parameters);

			// Post processing
			Logger.Info("Post processing...");

			var data = string.Empty;
			data += Write(result.UnnamedEnumValues);
			data += Write(result.GlobalVariables);
			data += Write(result.Structs);
			data += Write(result.StructDefaults);
			data += Write(result.Functions);

			var sb = new StringBuilder();
			sb.AppendLine(string.Format("// Generated by Hebron at {0}", DateTime.Now));
			sb.AppendLine();

			sb.AppendLine("use std;");
			sb.AppendLine("use c_runtime;");
			sb.AppendLine("use crate::*;");


			sb.AppendLine();
			sb.Append(data);

			File.WriteAllText(@"..\..\..\..\..\..\stb_image_write_rust\src\stb_image_write.rs", sb.ToString());
		}

		static void Main(string[] args)
		{
			try
			{
				Process();
			}
			catch (Exception ex)
			{
				Console.WriteLine(ex.Message);
				Console.WriteLine(ex.StackTrace);
			}

			Console.WriteLine("Finished. Press any key to quit.");
		}
	}
}