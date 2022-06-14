import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite

import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext
import java.text.SimpleDateFormat
import com.kms.katalon.core.configuration.RunConfiguration
import util.CustomReport
import com.kms.katalon.core.logging.model.TestStepLogRecord;
import com.aventstack.extentreports.MediaEntityBuilder
import com.aventstack.extentreports.MediaEntityModelProvider
import com.aventstack.extentreports.Status
import com.aventstack.extentreports.markuputils.ExtentColor
import com.aventstack.extentreports.markuputils.MarkupHelper
import Extbase.*;

class ExtentReport {
	/**
	 * Executes before every test case starts.
	 * @param testCaseContext related information of the executed test case.
	 */
	boolean ScriptExecuter = false 
	boolean isTC = false
	
	@BeforeTestCase
	def sampleBeforeTestCase(TestCaseContext testCaseContext) {
		println testCaseContext.getTestCaseId()
		println testCaseContext.getTestCaseVariables()
		CustomReport customReport = CustomReport.instance
		
		if (ScriptExecuter == false)
		{
		def date = new Date()
		def sdf = new SimpleDateFormat("dd_MM_yyyy_HHmmss")
		
		def start_time = sdf.format(date)
		
		
		// Get the project path
		
		//def relativeFilePath = projectPath.replace("\\","//");
		//def actualProjectPath = relativeFilePath+"//Reports//soapuiExecutionReport.html"
		//def actualProjectPath = "\\Extents\\soapuiExecutionReport"+start_time+".html"
		String dirName = RunConfiguration.getProjectDir()
		def file = new File(dirName + '\\Extents\\' + start_time)
		def success = file.mkdirs()
		GlobalVariable.ExtentHTML = dirName + '\\Extents\\' + start_time + "\\ExtentReport"+start_time+".html"
		GlobalVariable.ExtentFolder = dirName + '\\Extents\\' + start_time
		
		//customReport.initExtentReports("Extents/ExtentReport_GWS_suite.html")
		customReport.initExtentReports(GlobalVariable.ExtentHTML)
		
		ScriptExecuter = true
		isTC = true
		}
		customReport.startTestCase(testCaseContext.getTestCaseId())
		customReport.getExtentTest().log(Status.INFO,MarkupHelper.createLabel(testCaseContext.getTestCaseId() + "\t Started", ExtentColor.BLUE))
	}
	

	/**
	 * Executes after every test case ends.
	 * @param testCaseContext related information of the executed test case.
	 */
	@AfterTestCase
	def sampleAfterTestCase(TestCaseContext testCaseContext) {
		println testCaseContext.getTestCaseId()
		println testCaseContext.getTestCaseStatus()
		//TestStepLogRecord StepLog = testCaseContext.
		if (isTC == true)
		{
		CustomReport customReport = CustomReport.instance
		customReport.tearDownExtentReports()
		}
	}

	/**
	 * Executes before every test suite starts.
	 * @param testSuiteContext: related information of the executed test suite.
	 */
	@BeforeTestSuite
	def sampleBeforeTestSuite(TestSuiteContext testSuiteContext) {
		println testSuiteContext.getTestSuiteId()
		if (ScriptExecuter == false)
		{
		def date = new Date()
		def sdf = new SimpleDateFormat("dd_MM_yyyy_HHmmss")
		
		def start_time = sdf.format(date)
		
		
		// Get the project path
		
		//def relativeFilePath = projectPath.replace("\\","//");
		//def actualProjectPath = relativeFilePath+"//Reports//soapuiExecutionReport.html"
		//def actualProjectPath = "\\Extents\\soapuiExecutionReport"+start_time+".html"
		String dirName = RunConfiguration.getProjectDir()
		def file = new File(dirName + '\\Extents\\'  + start_time)
		def success = file.mkdirs()
		GlobalVariable.ExtentHTML = dirName + '\\Extents\\' +   start_time + "\\ExtentReport"+start_time+".html"
		GlobalVariable.ExtentFolder = dirName + '\\Extents\\' + start_time
		CustomReport customReport = CustomReport.instance
		//customReport.initExtentReports("Extents/ExtentReport_GWS_suite.html")
		customReport.initExtentReports(GlobalVariable.ExtentHTML)
		ScriptExecuter = true
		}
		
	}

	/**
	 * Executes after every test suite ends.
	 * @param testSuiteContext: related information of the executed test suite.
	 */
	@AfterTestSuite
	def sampleAfterTestSuite(TestSuiteContext testSuiteContext) {
		println testSuiteContext.getTestSuiteId()
		if (isTC == false)
			{
		CustomReport customReport = CustomReport.instance
		customReport.tearDownExtentReports()
		
			}
			def xyz = testSuiteContext.getTestSuiteId() + "#"+ testSuiteContext.status + "#" + testSuiteContext.getStatus()
			Extbase.abc = Extbase.abc + ";" + xyz
			
			println Extbase.abc
	}
}