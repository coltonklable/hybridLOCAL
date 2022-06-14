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
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.network.ProxyInformation as ProxyInformation
import com.kms.katalon.core.network.ProxyOption as ProxyOption
import com.kms.katalon.core.network.ProxyServerType as ProxyServerType
import com.kms.katalon.core.util.internal.JsonUtil as JsonUtil

import internal.GlobalVariable

public class Extbase {
	public static String abc = "";
	public static runBatchFile(String batchFile) {
		try {
			// Run "netsh" Windows command
			String bf = RunConfiguration.getProjectDir() + "/" + batchFile;
			Process process = Runtime.getRuntime().exec(bf);

			// Get input streams
			BufferedReader stdInput = new BufferedReader(new InputStreamReader(process.getInputStream()));
			BufferedReader stdError = new BufferedReader(new InputStreamReader(process.getErrorStream()));

			// Read command standard output
			String s;
			System.out.println("Standard output: ");
			while ((s = stdInput.readLine()) != null) {
				System.out.println(s);
			}

			// Read command errors
			System.out.println("Standard error: ");
			while ((s = stdError.readLine()) != null) {
				System.out.println(s);
			}
		} catch (Exception e) {
			e.printStackTrace(System.err);
		}
	}

	public static setManualProxy() {
		ProxyInformation proxyInfo = new ProxyInformation()
		proxyInfo.setProxyServerAddress('atlwsproxy.galileo.corp.lcl')
		proxyInfo.setProxyServerPort(8080)
		proxyInfo.setUsername('divya.anand')
		proxyInfo.setPassword('0M$ai#01')
		proxyInfo.setProxyOption(ProxyOption.MANUAL_CONFIG.toString())
		proxyInfo.setProxyServerType(ProxyServerType.HTTP.toString())
		// update Proxy settings
		RunConfiguration.getExecutionGeneralProperties().put(RunConfiguration.PROXY_PROPERTY, JsonUtil.toJson(proxyInfo))
		// retrieve Proxy settings
		println(RunConfiguration.getProxyInformation())
	}

	public static setNoProxy() {
		ProxyInformation proxyInfo = new ProxyInformation()
		proxyInfo.setProxyOption(ProxyOption.NO_PROXY.toString())
		proxyInfo.setProxyServerType(ProxyServerType.HTTP.toString())
		// update Proxy settings
		RunConfiguration.getExecutionGeneralProperties().put(RunConfiguration.PROXY_PROPERTY, JsonUtil.toJson(proxyInfo))
		// retrieve Proxy settings
		println(RunConfiguration.getProxyInformation())

	}

	public static GetCoordinates_ofText_AppWindow(String WindowText , String sText) {
		int i = 1
		int Find = 0
		int Row
		int ClassInx
		WindowText.eachLine {
			println it  + "line" + i

			if(Find == 0)
			{
				if (it.contains(sText))
				{
					String abc = it
					Row = i
					ClassInx = abc.indexOf(sText)
					println i + "," + ClassInx
					Find = 1

				}

			}
			i++
		}

		int x = (ClassInx * 7)+ 11
		int y = Row*13
		//int[] xy = new int[2]
		def xy = []
		//int[2] xy = { x, y }
		//int[] xy
		xy.add(x)
		xy.add(y)

		//println('Before')
		//return ((ClassInx* 7)+ 11) + "" ((Row)*13))
		return xy
	}

	public static GetCoordinates_ofText_PNRWindow(String WindowText , String sText) {
		int i = 1
		int Find = 0
		int Row
		int ClassInx
		WindowText.eachLine {
			println it  + "line" + i

			if(Find == 0)
			{
				if (it.contains(sText))
				{
					String abc = it
					Row = i
					ClassInx = abc.indexOf(sText)
					println i + "," + ClassInx
					Find = 1

				}

			}
			i++
		}


		int x = ((ClassInx +1) * 7)+ 11
		int y
		if (Row > 10)
		{
			y = ((Row)*14) + 15
		}
		else
		{
			y = (Row)*14
		}
		//int[] xy = new int[2]
		def xy = []
		//int[2] xy = { x, y }
		//int[] xy
		xy.add(x)
		xy.add(y)

		//println('Before')
		//return ((ClassInx* 7)+ 11) + "" ((Row)*13))
		return xy
	}


}

