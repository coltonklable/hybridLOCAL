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
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
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

CustomKeywords.'cAPI.Extbase.setManualProxy'()

GlobalVariable.StartDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyyMMdd', GlobalVariable.DateValue1)

GlobalVariable.DepartureDate = CustomKeywords.'cAPI.GenericLib.FDate'('yyyy-MM-dd', GlobalVariable.DateValue1)

Response1 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Air LFS', [('TargetBranch') : GlobalVariable.TargetBranch
			, ('Provider') : GlobalVariable.Provider, ('Origin') : GlobalVariable.Origin, ('Destination') : GlobalVariable.Destination
			, ('Carrier') : GlobalVariable.Carrier, ('DepartureDate') : GlobalVariable.DepartureDate]))

WS.verifyResponseStatusCode(Response1, 200)

String xml1 = Response1.responseText

def result1 = new XmlSlurper().parseText(xml1)

println('result1' + result1)

GlobalVariable.Carrier = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Carrier

println('Carrier' + GlobalVariable.Carrier)

String FlightNumber = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@FlightNumber

if (FlightNumber.length() < 4) {
	FlightNumber = ('0' + FlightNumber)
}

GlobalVariable.FlightNumber = FlightNumber

GlobalVariable.Origin = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Origin

GlobalVariable.Destination = result1.Body.LowFareSearchRsp.AirSegmentList.AirSegment[0].@Destination

Response2 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/BeginSession', [('Profile') : GlobalVariable.Profile1]))

WS.verifyResponseStatusCode(Response2, 200)

String xml2 = Response2.responseBodyContent

def result2 = new XmlSlurper().parseText(xml2)

GlobalVariable.TokenNumber = result2.BeginSessionResult.text()

println('TokenNumber' + GlobalVariable.TokenNumber)

Response3 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_Create PNR',
		[('TokenNumber') : GlobalVariable.TokenNumber, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider
			, ('Carrier') : GlobalVariable.Carrier, ('FlightNumber') : GlobalVariable.FlightNumber, ('Origin') : GlobalVariable.Origin
			, ('Destination') : GlobalVariable.Destination, ('StartDate') : GlobalVariable.StartDate, ('TicketDate') : GlobalVariable.TicketDate]))

WS.verifyResponseStatusCode(Response3, 200)

String xml3 = Response3.responseBodyContent

def result3 = new XmlSlurper().parseText(xml3)

GlobalVariable.RecordLoc = result3.SubmitXmlOnSessionResult.PNRBFManagement_49.EndTransaction.EndTransactResponse[0].RecLoc.text()

println('RecordLoc' + GlobalVariable.RecordLoc)

Response4 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_Add Received info',
		[('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response4, 200)

Response5 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/FareQuote', [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response5, 200)

Response6 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve1', [('Username') : GlobalVariable.Username
			, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response6, 200)

Response7 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRBFManagement_StorePrice',
		[('Username') : GlobalVariable.Username, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('Carrier') : GlobalVariable.Carrier
			, ('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response7, 200)

Response8 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve2', [('Username') : GlobalVariable.Username
			, ('PCC') : GlobalVariable.PCC1, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
			, ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response8, 200)

Response9 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Endsession', [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response9, 200)

Response10 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/BeginSession2', [('Profile') : GlobalVariable.Profile2]))

WS.verifyResponseStatusCode(Response10, 200)

String xml4 = Response10.responseBodyContent

def result4 = new XmlSlurper().parseText(xml4)

GlobalVariable.TokenNumber = result4.BeginSessionResult.text()

println('TokenNumber' + GlobalVariable.TokenNumber)

Response11 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/PNRRetrieve3', [('Username') : GlobalVariable.Username
	   , ('PCC') : GlobalVariable.PCC2, ('Provider') : GlobalVariable.Provider, ('TokenNumber') : GlobalVariable.TokenNumber
	   , ('RecordLoc') : GlobalVariable.RecordLoc]))

WS.verifyResponseStatusCode(Response11, 200)

Response12 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/DocProdFareManipulation',
		[('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response12, 200)

Response13 = WS.sendRequestAndVerify(findTestObject('XMLSelect_Galileo_Smoke/Customer Scenarios/HIS/Endsession2', [('TokenNumber') : GlobalVariable.TokenNumber]))

WS.verifyResponseStatusCode(Response13, 200)


