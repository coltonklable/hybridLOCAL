import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import travelport.CASTLE.*
import groovy.json.JsonSlurper as JsonSlurper
import groovy.util.XmlSlurper as XmlSlurper
import java.io.*
import java.util.*
import java.io.File as File

//println('From ' + From)
//println('To' + To)
response1 = WS.sendRequest(findTestObject('UAPI Ticketing flow/Air LFS', [('TargetBranch') : GlobalVariable.TargetBranch
            , ('Provider') : GlobalVariable.Provider]))

WS.verifyResponseStatusCode(response1, 200)

Result = response1.responseBodyContent

int index = Result.indexOf('<air:AirSegment ')

println('index' + index)

int index1 = Result.indexOf('</air:AirSegment>')

println('index1' + index1)

String segment1 = Result.substring(index, index1 + 17)

println "segment1" + segment1

String Segment_Split = segment1.remove('<air:FlightDetailsRef')

println "Segment_Split" + Segment_Split

String Seg = Segment_Split[0]

println "Seg" + Seg

String Seg_Split = Seg.split('<air:AirAvailInfo')

String SplitSeg = Seg_Split[0].split('>')

println "SplitSeg" + SplitSeg[0]

String insertStr = " ProviderCode=\"" + GlobalVariable.Provider + "\""

println "insertStr" + insertStr

String AddProvider = SplitSeg[0] + insertStr + ">" + "Seg_Split[1]"

GlobalVariable.AirSegment = AddProvider + "</air:AirSegment>"

println('AirSegment' + GlobalVariable.AirSegment)

/*response2 = WS.sendRequest(findTestObject('UAPI Ticketing flow/AirPrice Request', [('AirSegment') : GlobalVariable.AirSegment
            , ('TargetBranch') : GlobalVariable.TargetBranch]))

WS.verifyResponseStatusCode(response2, 200)

String xml1 = response2.responseBodyContent

def result1 = new XmlSlurper().parseText(xml1)
println('result1' + result1)
def Segment = xml1.AirItinerary[0].AirSegment
int index2 = xml1.indexOf('<air:AirSegment')

println('index2' + index2)

int index3 = xml1.indexOf('</air:AirSegment>')

println('index3' + index3)

String segment2 = xml1.substring(index2, index3 + 17)

println('Segment' + segment2)

int index4 = xml1.indexOf('<air:AirPricingSolution')

println('index4' + index4)

int index5 = xml1.indexOf('</air:AirPricingSolution>')

println('index5' + index5)

String PricingSolution = xml1.substring(index4, index5 + 25)

println('PricingSolution' + PricingSolution)

String PricingSolution_Split = PricingSolution.split('<air:AirSegmentRef')

println('PricingSolution_Split[0]' + (PricingSolution_Split[0]))

println('PricingSolution_Split[1]' + (PricingSolution_Split[1]))

String Pricing = (PricingSolution_Split[0]) + segment1

String PricingInfo = PricingSolution.substring(Airpricing.indexOf('<air:AirPricingInfo'))

GlobalVariable.PricingSol = (Pricing + PricingInfo)

println('Final PricingSolution' + GlobalVariable.PricingSol)

response3 = WS.sendRequest(findTestObject('UAPI Ticketing flow/AirBookReq', [('PricingSolution') : GlobalVariable.AirPricingSolution
            , ('TargetBranch') : GlobalVariable.TargetBranch]))

*/