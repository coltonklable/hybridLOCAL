package uapiFunctions

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import uapiFunctions.Libraries as UAPILib
import internal.GlobalVariable

public class CustomValidation {

	@Keyword
	public static boolean validateResponseToExpected(String strXMLDetails,
			String strRes_Xpath, String strValidationCategory, String strExpectedValue) {

		if (strXMLDetails.contains("<air:OptionalServicesTotal/>")) {
			strXMLDetails = strXMLDetails.replace("<air:OptionalServicesTotal/>", "");
		}

		char someChar = '_';
		int count = 0;

		for (int k = 0; k < strRes_Xpath.length(); k++) {
			if (strRes_Xpath.charAt(k) == someChar) {
				count++;
			}
		}
		if(count>1) {
			String [] UnderArray = strRes_Xpath.split("_");
			String sReplaceVersion = UnderArray[1];
			//strRes_Xpath = strRes_Xpath.replace(sReplaceVersion, "v"+XlsUtils.versionNumber);

		}

		String strRes_CapturedValue = "";
		String strStatus = "";

		String strReq_CapturedValue = strExpectedValue;

		if (strRes_Xpath.contains("[X]")) {
			String strRes_Xpathbuffer = strRes_Xpath;
			String strRes_CapturedValuebuffer = "";

			String[] strRes_Xpatharr = strRes_Xpath.split("\\[X]");
			String strRes_XpathNode = strRes_Xpatharr[0];
			strRes_Xpatharr = strRes_XpathNode.split("\\/");
			strRes_XpathNode = strRes_Xpatharr[strRes_Xpatharr.length - 1];

			String[] strNodeparts = strXMLDetails.split("<" + strRes_XpathNode);

			int intNodeparts = strNodeparts.length - 1;
			int intPass = 0;
			for (int Parts = 0; Parts < intNodeparts; Parts++) {
				int index = Parts + 1;
				strRes_Xpathbuffer = strRes_Xpath.replace("[X]", "[" + index
						+ "]");
				try {
					strRes_CapturedValuebuffer = UAPILib.retrieveValue(
							strXMLDetails, strRes_Xpathbuffer);
				} catch (Throwable t) {
				}
				if (strRes_CapturedValuebuffer.contains(strReq_CapturedValue)) {
					intPass = intPass + 1;
				}
			}

			if (strValidationCategory.equalsIgnoreCase("Existence")) {
				if (intNodeparts == intPass) {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue
					+ "</B> value is present in all <B>" + intPass
					+ "</B> nodes of " + strRes_XpathNode;
					strStatus = "Passed";
				} else {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue
					+ "</B> value is present ONLY in <B>" + intPass
					+ "</B> nodes out of " + intNodeparts
					+ " nodes of " + strRes_XpathNode;
					strStatus = "Failed";
				}
			} else if (strValidationCategory.equalsIgnoreCase("Non-Existence")) {
				String sResponse=strXMLDetails;
				if((sResponse.contains("faultcode")||sResponse.contains("faultstring")||sResponse.contains("Connection Timed Out")||sResponse.contains("TicketFailureInfo"))) {
					strStatus = "Failed";
					strRes_CapturedValue = "<B> Error message is received in the response </B>";
				}
				else {
					if (intPass == 0) {
						strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is NOT present in all <B>"+ intNodeparts + "</B> nodes of "	+ strRes_XpathNode;
						strStatus = "Passed";
					} else {
						strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is present in <B>" + intPass+ "</B> nodes out of " + intNodeparts	+ " nodes of " + strRes_XpathNode;
						strStatus = "Failed";
					}
				}
			} else if (strValidationCategory
			.equalsIgnoreCase("Atleast One Existence")) {

				if (intPass == 0) {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is NOT present in all <B>"+ intNodeparts + "</B> nodes of "+ strRes_XpathNode;
					strStatus = "Failed";
				} else {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is present in <B>" + intPass	+ "</B> nodes out of " + intNodeparts+ " nodes of " + strRes_XpathNode;
					strStatus = "Passed";
				}
			} else if (strValidationCategory.equalsIgnoreCase("Only One Existence")) {
				if (intPass == 1) {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is present Only once in <B>"+ intNodeparts + "</B> nodes of "	+ strRes_XpathNode;
					strStatus = "Passed";
				} else if (intPass == 0) {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is NOT present in <B>" + intNodeparts	+ "</B> nodes of " + strRes_XpathNode;
					strStatus = "Failed";
				} else {
					strRes_CapturedValue = "<B>" + strReq_CapturedValue	+ "</B> value is present More than Once (<B>" + intPass + "</B>) in " + intNodeparts+ " nodes of " + strRes_XpathNode;
					strStatus = "Failed";
				}
			}
		} else {

			strRes_CapturedValue = UAPILib.retrieveValue(strXMLDetails,
					strRes_Xpath);
			if (strRes_CapturedValue.contains(" - not found in the xml") || strRes_CapturedValue.equalsIgnoreCase("")) {
				strRes_CapturedValue = strRes_CapturedValue.replace(" - not found in the xml"," is NOT present in the response");

				strStatus = "Failed";
			} else {
				if (strRes_CapturedValue.equalsIgnoreCase(strExpectedValue)) {
					strStatus = "Passed";
				} else {
					strStatus = "Failed";
				}
			}
		}


		if(strStatus.equalsIgnoreCase("FAILED")) {
			//Logging.LogMessage(com.travelport.testfactory.Constants.giPass, "XML Node Verification", "XML Node: "+strReq_CapturedValue +" matched with actual value "+strRes_CapturedValue+" in response");
			println(strStatus+" "+strRes_CapturedValue)
			return false;
		}
		println(strStatus+" "+strRes_CapturedValue)
		return true;
	}
}
