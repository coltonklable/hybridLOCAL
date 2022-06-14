package util

import org.apache.commons.io.FileUtils
import org.apache.commons.io.FilenameUtils
import org.openqa.selenium.OutputType
import org.openqa.selenium.TakesScreenshot
import org.openqa.selenium.WebDriver

import com.aventstack.extentreports.ExtentReports
import com.aventstack.extentreports.ReportAggregates
import com.aventstack.extentreports.ReportConfigurator

import com.aventstack.extentreports.ExtentTest
import com.aventstack.extentreports.reporter.ExtentHtmlReporter

import com.aventstack.extentreports.reporter.configuration.Theme
import com.aventstack.extentreports.AnalysisStrategy
import com.kms.katalon.core.configuration.RunConfiguration as RC
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords
import java.text.SimpleDateFormat
import com.kms.katalon.core.configuration.RunConfiguration
import internal.GlobalVariable as GlobalVariable

@Singleton
public class CustomReport {

	private ExtentHtmlReporter extentHtmlReport
	private ExtentReports extentReport
	private ExtentTest testLogger
	private String currentDir
	private String fileSeperator
	private ExtentTest testStepLogger

	public ExtentTest getExtentTest() {
		return testLogger
	}

	public ExtentTest getExtentTestStep() {
		return testStepLogger
	}

	public void setExtentTest(ExtentTest testLogger) {
		this.testLogger =  testLogger
	}

	public ExtentReports initExtentReports(String filePath) {
		currentDir = System.getProperty('user.dir')
		fileSeperator = System.getProperty("file.separator")

		def date = new Date()
		def sdf = new SimpleDateFormat("dd_MM_yyyy_HHmmss")

		def start_time = sdf.format(date)
		String dirName = RunConfiguration.getProjectDir()
		def file = new File(dirName + '\\Extents\\DynamicSuite'  + start_time)
		////if(ScriptExecuter) {
		extentReport = new ExtentReports()
		//}
		extentHtmlReport = new ExtentHtmlReporter(FilenameUtils.separatorsToSystem(filePath))
		//extentHtmlReport.setAppendExisting(ReportConfigurator.app.isAppendExistingReport())
		//extentHtmlReport.setAppendExisting(true);
		//extentReport.config().se
		//extentReport = new ExtentReports(extentHtmlReport, false)
		//ExtentReports(extentReportFile, false)
		//extentHtmlReport.setAppendExisting(true);
		//		extentHtmlReport.config().setDocumentTitle("Automation Report")
		//		extentHtmlReport.config().setReportName("Functional Testing")
		//				extentHtmlReport.config().setTheme(Theme.DARK)
		extentReport.attachReporter(extentHtmlReport)

		extentHtmlReport.loadXMLConfig("${currentDir}${fileSeperator}Extents${fileSeperator}extent-config.xml")

		extentReport.setSystemInfo("OS", RC.getOS())
		extentReport.setSystemInfo("Host", RC.getHostAddress())
		extentReport.setSystemInfo("User", RC.getHostName())
		//extentReport.setSystemInfo("Host URL", WebUiBuiltInKeywords.getUrl())
		extentReport.setSystemInfo("Environment", RC.getExecutionProfile())
		extentReport.setSystemInfo("Katalon Version", RC.getAppVersion())
		extentReport.setAnalysisStrategy(AnalysisStrategy.SUITE);


	}

	public ExtentTest startTestStep(String testCaseName) {

		testStepLogger = testLogger.createNode(testCaseName)
		//testLogger = extentReport.createTest(testCaseName)



		return testStepLogger
	}

	public ExtentTest startTestCase(String testCaseName) {
		testLogger = extentReport.createTest(testCaseName)



		return testLogger
	}

	public String captureScreen() throws IOException {
		TakesScreenshot screen = (TakesScreenshot) DriverFactory.getWebDriver()
		File src = screen.getScreenshotAs(OutputType.FILE)
		String dest =GlobalVariable.ExtentFolder + new Date().format('dd_MM_yyy_hh_mm_ss') + ".png"
		File target = new File(dest)
		FileUtils.copyFile(src, target)
		return target.getAbsolutePath()
	}

	public String AttachRequestResponse(String ReqRes) throws IOException {

		//String sResponseFilePath = Path
		def date = new Date()
		def sdf = new SimpleDateFormat("dd_MM_yyyy_HHmmssSSS")

		def start_time = sdf.format(date)


		String XMLPath = GlobalVariable.ExtentFolder + "\\ReqRes"+start_time+".xml"


		def oResponseInputFile = new File(XMLPath)
		oResponseInputFile.write(ReqRes,"UTF-8")

		return XMLPath
	}

	public ExtentReports tearDownExtentReports() {
		extentReport.flush()
	}

	/*public String GetXMLPath() throws IOException {
	 def date = new Date()
	 def sdf = new SimpleDateFormat("dd_MM_yyyy_HHmmss")
	 def start_time = sdf.format(date)
	 String XMLPath = GlobalVariable.ExtentFolder + "\\ReqRes"+start_time+".xml"
	 return XMLPath
	 }*/
}
