package cAPI

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

import java.util.Calendar;
import java.util.Date;
import java.text.DateFormat;
import java.text.Format
import java.text.SimpleDateFormat;
import java.text.ParseException;

import internal.GlobalVariable
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.StringReader;
import java.io.StringWriter;
import java.util.Collections;
import java.util.List;
import java.util.Map;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.transform.OutputKeys;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import javax.xml.xpath.XPath;
import javax.xml.xpath.XPathConstants;
import javax.xml.xpath.XPathFactory;

import org.apache.commons.io.FileUtils;
import org.apache.commons.lang.StringUtils;
import org.w3c.dom.Document;
import org.w3c.dom.Node;
import org.xml.sax.InputSource;

import com.kms.katalon.core.testobject.impl.HttpTextBodyContent;
import com.kms.katalon.util.DocumentBuilderProvider;
import com.kms.katalon.util.TransformerFactoryProvider;
import com.kms.katalon.core.testobject.PerformanceResourceTiming
import com.kms.katalon.core.testobject.HttpMessage
import com.kms.katalon.core.testobject.HttpBodyContent

public class GenericLib {

	@Keyword

	public static FDate(String sFormat, Integer iFutureDays) {
		Calendar oCalender = Calendar.getInstance();
		String sDate = "Asked for Undefined Format";
		Format oFormatter;
		Date oDate = new Date();

		//** 14Jul14 (MC) added one more format("yyyy-MM")
		def dateFormatList = [
			"ddMMM",
			"ddMMMyy",
			"ddMMMyyyy",
			"MM/dd/yyyy",
			"MMddyyyy",
			"yyyy-MM-dd",
			"MM/dd/yy",
			"dd/MM/yy",
			"dd/MM/yyyy",
			"M/d/yyyy",
			"M/d/yy",
			"d/M/yyyy",
			"yyyyMMdd",
			"d/M/yy",
			"yyyy-MM"
		]
		try
		{
			if (dateFormatList.contains( sFormat ))
			{
				oFormatter = new SimpleDateFormat(sFormat);
				sDate = oFormatter.format(oDate);
				oCalender.setTime(oFormatter.parse(sDate));
				oCalender.add(Calendar.DATE, iFutureDays);
				sDate = oFormatter.format(oCalender.getTime());
			}
		}
		catch (Exception e)
		{
		}
		return sDate;
	}

	@Keyword

	@Deprecated
	private String responseText;
	public String responseBodyContentData() throws Exception {
		if (responseText != null) {
			if (contentType != null && contentType.startsWith("application/xml")) {
				DocumentBuilder db = DocumentBuilderProvider.newBuilderInstance();
				Document doc = db.parse(new InputSource(new StringReader(responseText)));
				XPath xPath = XPathFactory.newInstance().newXPath();
				String expression = "//Body";
				Node node = (Node) xPath.compile(expression).evaluate("//*//*//*", doc, XPathConstants.NODE);
				return nodeToString(node);
			} else if (contentType != null && contentType.startsWith("application/json")) {
				return responseText;
			}
			// plain text/html
			else {
				return responseText;
			}
		}
		return "";

	}


}
