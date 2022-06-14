package uapiFunctions
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException


class Libraries {
	/**
	 * Send request and verify status code
	 * @param request request object, must be an instance of RequestObject
	 * @param expectedStatusCode
	 * @return a boolean to indicate whether the response status code equals the expected one
	 */
	@Keyword
	def verifyStatusCode(TestObject request, int expectedStatusCode) {
		if (request instanceof RequestObject) {
			RequestObject requestObject = (RequestObject) request
			ResponseObject response = WSBuiltInKeywords.sendRequest(requestObject)
			if (response.getStatusCode() == expectedStatusCode) {
				KeywordUtil.markPassed("Response status codes match")
			} else {
				KeywordUtil.markFailed("Response status code not match. Expected: " +
						expectedStatusCode + " - Actual: " + response.getStatusCode() )
			}
		} else {
			KeywordUtil.markFailed(request.getObjectId() + " is not a RequestObject")
		}
	}

	/**
	 * Add Header basic authorization field,
	 * this field value is Base64 encoded token from user name and password
	 * @param request object, must be an instance of RequestObject
	 * @param username username
	 * @param password password
	 * @return the original request object with basic authorization header field added
	 */
	@Keyword
	def addBasicAuthorizationProperty(TestObject request, String username, String password) {
		if (request instanceof RequestObject) {
			String authorizationValue = username + ":" + password

			authorizationValue = "Basic " + authorizationValue.bytes.encodeBase64().toString()

			// Find available basic authorization field and change its value to the new one, if any
			List<TestObjectProperty> headerProperties = request.getHttpHeaderProperties()
			boolean fieldExist = false
			for (int i = 0; i < headerProperties.size(); i++) {
				TestObjectProperty headerField = headerProperties.get(i)
				if (headerField.getName().equals('Authorization')) {
					println(headerField.getValue())
					KeywordUtil.logInfo("Found existent basic authorization field. Replacing its value.")
					headerField.setValue(authorizationValue)
					println(authorizationValue)
					println(headerField.getValue())
					fieldExist = true
					break
				}
			}

			if (!fieldExist) {
				TestObjectProperty authorizationProperty = new TestObjectProperty("Authorization",
						ConditionType.EQUALS, authorizationValue, true)
				headerProperties.add(authorizationProperty)
			}
			TestObjectProperty headerField = headerProperties.get(0)
			println(headerField.getValue())
			//request.setHttpHeaderProperties(headerProperties)
			println(request.getHttpHeaderProperties().size())
			KeywordUtil.markPassed("Basic authorization field has been added to request header")
		} else {
			KeywordUtil.markFailed(request.getObjectId() + "is not a RequestObject")
		}
		return request
	}
	@Keyword
	public static String Create(String xml, String SearchString, String EndString) {

		String Res2 = new String();
		String sss = new String();
		sss = "";
		String search = "<" + SearchString;
		String searchend = "</" + EndString + ">";
		int index;
		int ind;
		Res2 = xml.replaceAll(">\\s*<", "><");
		int rs = Res2.length();
		index = Res2.indexOf(search);
		sss = Res2.substring(index, rs);
		index = sss.indexOf(">");
		sss = sss.substring(index, sss.length());
		int m = sss.length();
		index = sss.indexOf(">");
		String ss = sss.substring(index + 1, m);
		ind = ss.indexOf(searchend);
		String s = ss.substring(0, ind);
		return s;

	}

	@Keyword
	public static String retrieveValue(String xml, String xpath) {
		//air:AirPricingSolution [1]/air:AirPricingSolution
		xpath = xpath.replace("//", ""); //--air:AirPricingSolution [1]/air:AirPricingSolution
		xpath = xpath.replaceAll(">\\s*<", "><");// --air:AirPricingSolution [1]/air:AirPricingSolution
		String strXML = xml;
		String strStatus = "Pass";
		String strEndTag = "";
		String retStr = "";
		String strNode = null;
		String strNodeIndex = null;
		String endNode = null;
		String[] xdet = xpath.split("/");//2 elements

		int intcheckxpath = xdet.length - 1;
		String strcheckNodePath = xdet[intcheckxpath]; //air:AirPricingSolution
		if (strcheckNodePath.contains("@")) {
			String[] strcheckNodePatharr = strcheckNodePath.split("@");
			strcheckNodePath = strcheckNodePatharr[1];
		}
		if (strcheckNodePath.contains("[")) {
			String[] strcheckNodePatharr = strcheckNodePath.split("\\[");
			strcheckNodePath = strcheckNodePatharr[0];
		}
		strcheckNodePath = strcheckNodePath.replace(" ", "");

		if (strXML.contains(strcheckNodePath)) {
			for (int i = 0; i < xdet.length - 1; i++) {
				String strNodePath = xdet[i];//air:AirPricingSolution [1]/
				String[] strNodeSplit = strNodePath.split("\\[");
				strNode = strNodeSplit[0];
				strNodePath = strNodeSplit[1];
				strNodeSplit = strNodePath.split("\\]");
				strNodeIndex = strNodeSplit[0];
				strEndTag = "False";
				if (strXML.contains(strNode.trim())) {
					String[] strXMLsplit = strXML.split("<" + strNode);
					int intNodeIndex = Integer.parseInt(strNodeIndex);
					if (strXMLsplit.length > intNodeIndex) {
						retStr = strXMLsplit[intNodeIndex];
						strXML = retStr;
						endNode = strNode.trim();
						if (strXML.contains("</" + endNode)) {
							strXMLsplit = strXML.split("</" + endNode + ">");
							strXML = strXMLsplit[0];
							strEndTag = "True";
						} else {
							strXMLsplit = strXML.split("/>");
							strXML = strXMLsplit[0];
							strEndTag = "False";
						}
					} else {
						strStatus = strNode + "[" + intNodeIndex + "] - not found in the xml";
					}
				} else {
					strStatus = strNode + " - not found in the xml";
				}
			}

			if (strStatus.equalsIgnoreCase("Pass")) {
				int intLastxpath = xdet.length - 1;
				String strNodePath = xdet[intLastxpath];

				if (strNodePath.contains("E@")) {
					String[] strElementSplit = strXML.split(">");
					String strElementSplitvalue = strElementSplit[1];

					String[] strAttributeValueSplit = strElementSplitvalue.split("<");
					if(strElementSplitvalue.contains("![CDATA["))
					{
						try {
							retStr = strAttributeValueSplit[1].replaceAll("CDATA","").replaceAll("[!\\[\\[|\\]]", "");
						}
						catch(Exception e){
							e.printStackTrace();
						}
					}

					else
						retStr = strAttributeValueSplit[0];
				} else if (strNodePath.contains("@")) {
					String strAttribute = strNodePath.replace("@", "");
					if (strXML.contains(strAttribute)) {
						String[] strAttributeSplit = strXML.split(strAttribute + "=");
						String strAttributevalue = strAttributeSplit[1];
						if (!strAttributevalue.equalsIgnoreCase("\"\"")) {
							String[] strAttributeValueSplit = strAttributevalue.split("\"");
							retStr = strAttributeValueSplit[1];
						} else {
							retStr = "'";
						}

					} else {
						retStr = strAttribute + " - not found in the xml";
					}
				} else if (strNodePath.contains("[")) {

					String[] strNodeSplit = strNodePath.split("\\[");
					strNode = strNodeSplit[0];
					strNodePath = strNodeSplit[1];
					strNodeSplit = strNodePath.split("\\]");
					strNodeIndex = strNodeSplit[0];
					if (strXML.contains(strNode)) {
						String[] strXMLsplit = strXML.split("<" + strNode);
						int intNodeIndex = Integer.parseInt(strNodeIndex);
						if (strXMLsplit.length > intNodeIndex) {
							retStr = strXMLsplit[intNodeIndex];
							strXML = retStr;
							endNode = strNode.trim();
							retStr = "<" + strNode + retStr;
						}
					}
				} else {
					if (strEndTag.equalsIgnoreCase("True")) {
						retStr = "<" + strNodePath + strXML;
						retStr = retStr + "</" + endNode + ">";

					} else if (strEndTag.equalsIgnoreCase("False")) {
						retStr = "<" + strNodePath + strXML;
						retStr = retStr + "/>";
					} else {
						if (strXML.contains(strNodePath)) {
							String[] strXMLsplit = strXML.split("<" + strNodePath);
							if (strXMLsplit.length > intLastxpath) {
								retStr = strXMLsplit[1];
								strXML = retStr;
								retStr = "<" + strNodePath + retStr;
							}
						}
					}
				}
			} else {
				retStr = strStatus;
			}
		} else {
			retStr = strcheckNodePath + " - not found in the xml";
		}

		return retStr;


	}
	@Keyword
	public static String GetAirSegments(String xmlRsp,String gProvider) {


		String sResult = "";
		String xmlDetails = xmlRsp;

		String START_PATTERN1 = "<air:AirSegmentRef Key=\"";
		String END_PATTERN1 = "\"/>";
		String xpathPricing1 = "//air:AirPricingSolution [1]/air:AirPricingSolution ";
		String finalStr = retrieveValue(xmlDetails, xpathPricing1);
		int startIndex1 = -1, endIndex1 = -1;
		int fromIndex1 = 0;
		String get123 = "";
		while ((startIndex1 = finalStr.indexOf(START_PATTERN1, fromIndex1)) > -1) {
			endIndex1 = finalStr.indexOf(END_PATTERN1, startIndex1) + END_PATTERN1.length();
			int trimfront = startIndex1 + START_PATTERN1.length();
			int trimend = endIndex1 - END_PATTERN1.length();

			get123 = get123 + finalStr.substring(trimfront, trimend);
			fromIndex1 = endIndex1;
		}

		String[] key = get123.split("==");

		String providerCode = "";
		providerCode = gProvider;

		String strData = Create(xmlDetails, "air:AirSegmentList", "air:AirSegmentList");

		String strfulldata = strData;
		String strSeperdata = "";
		for (int i = 0; i < key.length; i++) {

			String START_PATTERN = "Key=\"" + key[i] + "==";
			String[] strfulldataarr = strfulldata.split("</air:AirSegment>");
			for (int k = 0; k < strfulldataarr.length; k++) {
				String strINDAir = strfulldataarr[k];
				if (strINDAir.contains(START_PATTERN)) {

					String[] strfulldataFlightarr = strINDAir.split("<air:FlightDetailsRef");
					String strINDAirremove = strfulldataFlightarr[0];

					String insertStr = " ProviderCode=\"" + providerCode + "\"";
					int keyindex = strINDAirremove.indexOf(START_PATTERN);
					String segment = strINDAirremove.substring(0, keyindex - 1) + " " + insertStr + " " + strINDAirremove.substring(keyindex, strINDAirremove.length());

					strSeperdata = strSeperdata + segment + "</air:AirSegment>";
					break;
				}
			}
		}
		int counttoken2 = 0;
		int idx2 = 0;
		while ((idx2 = strSeperdata.indexOf("<air:AirSegment", idx2)) != -1) {
			idx2++;
			counttoken2++;
		}
		String classRepl = "";
		String chkKey = "";

		if (strSeperdata.contains("ClassOfService")) {

		} else {
			String xpathPassive = "//air:AirSegment[1]/air:BookingCodeInfo [1]/@BookingCounts";
			String chkOutput = retrieveValue(strSeperdata, xpathPassive);

			// Form 15.4 to Next release
			if (chkOutput.contains(" not found ")) {
				xpathPassive = "//air:AirPricingInfo [1]/air:BookingInfo [1]/@BookingCode";
				chkOutput = retrieveValue(xmlDetails, xpathPassive);
			}
			classRepl = chkOutput.substring(0, 1);
			strSeperdata = strSeperdata.replace("<air:AirSegment ",
					"<air:AirSegment ClassOfService=\"" + classRepl + "\" ");

		}

		sResult =strSeperdata;
		return (sResult);

	}
	@Keyword
	public static String getPricingSolutions(String xmlRsp) {

		String xmlDetails = retrieveValue(xmlRsp, "air:AirPriceRsp");
		String sResult = "";
		if (xmlDetails.contains("<air:FeeInfo")) {
			int countfee = 0;
			int idxfee = 0;
			while ((idxfee = xmlDetails.indexOf("<air:FeeInfo", idxfee)) != -1) {
				idxfee++;
				countfee++;
			}
			for (int z = 1; z <= countfee; z++) {
				String xpathFee = "//air:FeeInfo [1]/air:FeeInfo ";
				String strFee = retrieveValue(xmlDetails, xpathFee);
				xmlDetails = xmlDetails.replace(strFee, "");
			}
		}

		if (xmlDetails.contains("<air:BrandingInfo") || xmlDetails.contains("<air:Brand ")) {
			xmlDetails = RemoveBrandingInfo(xmlDetails);
		}
		String START_PATTERN = "<air:PassengerType";
		String END_PATTERN = "/>";
		String xml = xmlDetails;
		String AirSegmentNew = "";
		String out = "";
		String Airpricing = Create(xml, "air:AirPriceResult", "air:AirPricingSolution");

		String AirSegment = Create(xml, "air:AirItinerary", "air:AirItinerary");

		String[] Airpricing_split = Airpricing.split("<air:AirSegmentRef");

		if (AirSegment.contains("<common:HostToken")) {
			String[] AirSegment1 = AirSegment.split("<common:HostToken");
			AirSegmentNew = AirSegment1[0];
			out = Airpricing_split[0] + AirSegmentNew;
		} else {
			out = Airpricing_split[0] + AirSegment;
		}

		String Airpricing2=Airpricing.substring(Airpricing.indexOf("<air:AirPricingInfo"));
		out=out+Airpricing2;
		int count = 1;
		int startIndex = -1, endIndex = -1, fromIndex = 0;

		while ((startIndex = out.indexOf(START_PATTERN, fromIndex)) > -1) {
			endIndex = out.indexOf(END_PATTERN, startIndex);

			fromIndex = endIndex;
			String insertStr = " BookingTravelerRef=\"" + (count++) + "\"";

			out = out.substring(0, fromIndex) + insertStr + out.substring(endIndex, out.length());
		}

		String Result = out;
		int count1 = 0;
		int idx1 = 0;
		while ((idx1 = Result.indexOf("<air:FareInfo ", idx1)) != -1) {
			idx1++;
			count1++;
		}
		for (int z = 1; z <= count1; z++) {
			String xpathBrand = "//air:FareInfo [" + z + "]/air:Brand [1]/air:OptionalServices[1]/air:OptionalServices";
			String strBrand = retrieveValue(Result, xpathBrand);
			Result = Result.replace(strBrand, "");

		}
		if (Result.contains("<air:OptionalServices")) {
			int hostStart = Result.indexOf("<air:OptionalServices");
			Result = Result.substring(0, hostStart);
		}

		String apir = "";
		if (Result.contains("<air:APISRequirements")) {
			int hostStart = Result.indexOf("<air:APISRequirements");
			if (Result.contains("</air:APISRequirements")) {
				int hostEnd = Result.indexOf("</air:APISRequirements>");
				apir = Result.substring(hostStart, hostEnd + 23);
			} else {
				int hostEnd = Result.indexOf("/>", hostStart);
				apir = Result.substring(hostStart, hostEnd + 2);
			}
			Result = Result.replace(apir, "");
		}
		sResult = Result + "</air:AirPricingSolution>";
		return (sResult);

	}
	@Keyword
	public static String RemoveBrandingInfo(String xmlDetails) {

		int count1 = 0;
		int idx1 = 0;
		while ((idx1 = xmlDetails.indexOf("<air:AirPricingSolution ", idx1)) != -1) {
			idx1++;
			count1++;
		}

		if (count1 >= 2) {
			String xpathPrice2 = "//air:AirPricingSolution [2]/air:AirPricingSolution ";
			String strPrice2 = retrieveValue(xmlDetails, xpathPrice2);
			xmlDetails = xmlDetails.replace(strPrice2, "");
		}

		if (xmlDetails.contains("<air:BrandingInfo")) {
			int count = 0;
			int idx = 0;
			while ((idx = xmlDetails.indexOf("<air:BrandingInfo", idx)) != -1) {
				idx++;
				count++;
			}
			for (int z = 1; z <= count; z++) {
				String xpathBrand = "//air:BrandingInfo [1]/air:BrandingInfo ";
				String strBrand = retrieveValue(xmlDetails, xpathBrand);
				xmlDetails = xmlDetails.replace(strBrand, "");
			}
		}

		if (xmlDetails.contains("<air:Brand")) {
			int count = 0;
			int idx = 0;
			while ((idx = xmlDetails.indexOf("<air:Brand ", idx)) != -1) {
				idx++;
				count++;
			}
			for (int z = 1; z <= count; z++) {
				String xpathBrand = "//air:Brand [" + z + "]/air:Brand ";
				String strBrand = retrieveValue(xmlDetails, xpathBrand);

				if (strBrand.contains("<air:OptionalServices")) {
					String xpathOpt = "//air:Brand [1]/air:OptionalServices[1]/air:OptionalServices";
					String Opt = retrieveValue(strBrand, xpathOpt);
					xmlDetails = xmlDetails.replace(Opt, "");
				}
			}
		}

		if (xmlDetails.contains("<air:OptionalServicesTotal/>")) {
			xmlDetails = xmlDetails.replace("<air:OptionalServicesTotal/>", "");
		}

		return (xmlDetails);

	}
	@Keyword
	public static String GetOptionalServices(String xmlRsp) {

		String sResult = "";
		String sServiceData = "";
		String xpathOptService;
		String OptService = null;
		String OptServices = null;
		String newOptService = "";
		String xmlDetails = xmlRsp;
		int i = 1;

		xpathOptService = "//air:OptionalServices[1]/air:OptionalServices";
		OptServices = retrieveValue(xmlDetails, xpathOptService);
		// while(( i = xmlDetails.indexOf("<air:OptionalService", i)) != -1)
		while (i >= 1 && !OptServices.contains("not found in the xml")) {
			xpathOptService = "//air:OptionalService [" + i + "]/air:OptionalService";

			OptService = retrieveValue(xmlDetails, xpathOptService);

			if (!OptService.contains("PreReservedSeatAssignment")) {
				newOptService = OptService;
				break;
			}

			else if (OptService.contains("not found in the xml")) {
				break;
			} else {
				++i;
			}
		}


		sResult = newOptService.replace("<air:OptionalService", "<air:OptionalService ");


		return (sResult);

	}

	@Keyword
	public static String GetAirSegmentMerchOffer(String xmlRsp) {

		String FinalResult = "";
		String xmlDetails = xmlRsp;
		int count = 0;
		int idx = 0;
		int count1 = 0;
		int idx1 = 0;
		while ((idx = xmlDetails.indexOf("<air:AirSegment ", idx)) != -1) {
			idx++;
			count++;
		}

		for (int i = 1; i <= count; i++) {
			String xpathSeg = "//air:AirSegment [" + i + "]/air:AirSegment ";
			String Seg = retrieveValue(xmlDetails, xpathSeg);

			if (Seg.contains("ProviderReservationInfoRef")) {
				String xpathProRef = "//air:AirSegment [" + i + "]/@ ProviderReservationInfoRef";
				String ProRef = retrieveValue(xmlDetails, xpathProRef);

				Seg = Seg.replace("ProviderReservationInfoRef=\"" + ProRef + "\"", "");
			}

			FinalResult = FinalResult + Seg;
		}
		while ((idx1 = xmlDetails.indexOf("<air:SearchTraveler ", idx1)) != -1) {
			idx1++;
			count1++;
		}
		for (int i = 1; i <= count1; i++) {
			String xpathTrav = "//air:SearchTraveler [" + i + "]/air:SearchTraveler ";
			String Trav = retrieveValue(xmlDetails, xpathTrav);
			FinalResult = Trav + FinalResult;
		}
		return (FinalResult);

	}

	@Keyword
	public static String GetSeatDataMPS(String xmlDetails, String Result, String seatmapcnt) {

		String Filepath = "";
		String FilepathReturn = "";

		String URxml = "";
		int count1 = 0;
		int count2 = 0;
		int idx = 0;
		int idx1 = 0;
		String xpathPaid = "";
		String strPaid = "";
		String xpathSeatPrice = "";
		String strSeatPrice = "";
		String xpathSeatCode = "";
		String strSeatCode = "";
		String flag1 = "false";
		String strTravRef;

		int idx2 = 0;
		int countTrav = 0;
		while ((idx2 = xmlDetails.indexOf("</air:SearchTraveler>", idx2)) != -1) {
			idx2++;
			countTrav++;

		}

		String[] seatcodeArr = new String[countTrav];
		String[] seatpriceArr = new String[countTrav];

		while ((idx = xmlDetails.indexOf("<air:Row ", idx)) != -1) {
			idx++;
			count1++;
		}
		int copycountTrav = countTrav;
		int flg = 1;
		int cntTraCopy = 1;

		for (int cntTra = 1; cntTra <= countTrav; cntTra++) {
			cntTraCopy = cntTra;
			String xpathSeaTra = "//air:SeatMapRsp [1]/air:SearchTraveler [" + cntTra + "]/@Key";
			String strSeaTra = retrieveValue(xmlDetails, xpathSeaTra);
			for (int z = 1; z <= count1; z++) {
				String xpathRow = "//air:Row [" + z + "]/air:Row ";
				String xpathTrav = "//air:Row [" + z + "]/@SearchTravelerRef";
				URxml = retrieveValue(xmlDetails, xpathRow);
				String travVal = retrieveValue(xmlDetails, xpathTrav);
				String xpathSeaTraCode = "//air:SeatMapRsp [1]/air:SearchTraveler [" + cntTra + "]/@Code";
				String strSeaTraCode = retrieveValue(xmlDetails, xpathSeaTraCode);
				if (strSeaTraCode.equalsIgnoreCase("INF")) {
					flg = 100;
				}
				if (flg == 100) {
					cntTraCopy = cntTra - 1;
					copycountTrav = cntTraCopy;
				}

				// System.out.println(travVal);
				if (travVal.contains(strSeaTra)) {

					if (flag1.equalsIgnoreCase("true")) {
						continue;
					} else {
						while ((idx1 = URxml.indexOf("<air:Facility ", idx1)) != -1) {
							idx1++;
							count2++;
						}
						for (int k = 1; k <= count2; k++) {
							String xpathAvail = "//air:Facility [" + k + "]/@Availability";
							String availVal = retrieveValue(URxml, xpathAvail);
							if (availVal.equalsIgnoreCase("Available")) {
								xpathPaid = "//air:Facility [" + k + "]/@Paid";
								strPaid = retrieveValue(URxml, xpathPaid);
								xpathSeatPrice = "//air:Facility [" + k + "]/@SeatPrice";
								strSeatPrice = retrieveValue(URxml, xpathSeatPrice);
								xpathSeatCode = "//air:Facility [" + k + "]/@SeatCode";
								strSeatCode = retrieveValue(URxml, xpathSeatCode);
								strSeatCode = strSeatCode.replace("-", "");
								if ((!strPaid.contains("not found in the xml"))
								&& !strSeatPrice.contains("not found in the xml")) {
									if ((!strSeatCode.contains("not found in the xml"))) {

										strSeatCode = strSeatCode.replace("-", "");
										seatcodeArr[cntTraCopy - 1] = strSeatCode;
										seatpriceArr[cntTraCopy - 1] = strSeatPrice;

										if (cntTraCopy == 2) {
											if (seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 2])) {
												continue;
											}
										}
										if (cntTraCopy == 3) {
											if (seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 2])
											|| seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 3])) {
												continue;
											}
										}
										if (cntTraCopy == 4) {
											if (seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 2])
											|| seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 3])
											|| seatcodeArr[cntTraCopy - 1]
											.equalsIgnoreCase(seatcodeArr[cntTraCopy - 4])) {
												continue;
											}
										}

										flag1 = "true";
										break;
									}
								}
							}
						}
						count2 = 0;
					}
				}

				if (flag1.equalsIgnoreCase("true")) {
					break;
				}
			}
			flag1 = "false";
		}
		String xml1 = Result;
		String val = strSeatCode;
		// String Result = xml1;
		println("val is "+ val)

		if (seatmapcnt.equalsIgnoreCase("0")) {
			for (int z = 1; z <= copycountTrav; z++) {
				String xpathTravRef = "//air:OptionalService [" + z + "]/common_v51_0:ServiceData [1]/@BookingTravelerRef";
				strTravRef = retrieveValue(xml1, xpathTravRef);
				println("taverler ref"+ strTravRef)

				/*
				 * Result = Result.replace(" BookingTravelerRef=\"" + strTravRef + "\"/>",
				 * " BookingTravelerRef=\"" + strTravRef + "\" Data=\"" + strSeatCode + "\"/>");
				 */
				Result = Result.replace(" BookingTravelerRef=\"" + strTravRef + "\""," BookingTravelerRef=\"" + strTravRef + "\" Data=\"" + strSeatCode + "\"");
				println("result is"+Result)
				/*
				 * String xpathTotPrice = "//air:OptionalService [" + z + "]/@ TotalPrice";
				 * String strTotalPrice = retrieveValue(xml1, xpathTotPrice); Result =
				 * Result.replace(" TotalPrice=\"" + strTotalPrice + "\"", " TotalPrice=\"" +
				 * seatpriceArr[z - 1] + "\"");
				 */

			}
		}


		return (Result);

	}
	@Keyword
	public static String GetOptionalServicesWithSeatMap(String prevResponse) {

		String sResult = "";
		String sServiceData = "";
		String xpathOptService;
		String OptService = null;
		String OptServices = null;
		String newOptService = "";
		String xmlDetails = prevResponse;
		int i = 1;

		xpathOptService = "//air:OptionalServices[1]/air:OptionalServices";
		OptServices = retrieveValue(xmlDetails, xpathOptService);
		// while(( i = xmlDetails.indexOf("<air:OptionalService", i)) != -1)
		while (i >= 1 && !OptServices.contains("not found in the xml")) {
			xpathOptService = "//air:OptionalService [" + i + "]/air:OptionalService";
			String xpathOptService1 = "//air:Row [1]/air:Row";
			OptService = retrieveValue(xmlDetails, xpathOptService1);
			OptService = retrieveValue(xmlDetails, xpathOptService);
			if (OptService.contains("PreReservedSeatAssignment")) {
				newOptService = OptService;
				break;

			}

			else if (OptService.contains("not found in the xml")) {
				break;
			} else {
				++i;
			}
		}


		sResult = newOptService.replace("<air:OptionalService", "<air:OptionalService ");



		sServiceData = GetSeatDataMPS(xmlDetails, sResult, "0");
		//sResult = "<air:OptionalServices>" + sServiceData + "</air:OptionalServices>";
		sResult =  sServiceData ;


		return (sResult);

	}
}