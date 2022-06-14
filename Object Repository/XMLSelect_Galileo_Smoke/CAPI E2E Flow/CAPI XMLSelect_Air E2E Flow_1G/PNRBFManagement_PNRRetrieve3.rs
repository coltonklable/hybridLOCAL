<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PNRBFManagement_PNRRetrieve3</name>
   <tag></tag>
   <elementGuidId>e4dbb66a-db85-4555-988a-489bae28ce7a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R1dTX1VBVF9QNzExMzE2MDpOZXdwYXNzXzR5b3U=</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXml>
         &lt;web:Profile>${Profile}&lt;/web:Profile>
         &lt;web:Request>
           &lt;PNRBFManagement_53>
               &lt;PNRBFRetrieveMods>
                  &lt;PNRAddr>
                     &lt;RecLoc>${RecordLoc}&lt;/RecLoc>
                  &lt;/PNRAddr>
                  &lt;TargetCRSInfo>
                     &lt;TargetCRS>${Provider}&lt;/TargetCRS>
                  &lt;/TargetCRSInfo>
               &lt;/PNRBFRetrieveMods>
               &lt;FareRedisplayMods>
                  &lt;DisplayAction>
                     &lt;Action>D&lt;/Action>
                  &lt;/DisplayAction>
                  &lt;FareNumInfo>
                     &lt;FareNumAry>
                        &lt;FareNum>1&lt;/FareNum>
                     &lt;/FareNumAry>
                  &lt;/FareNumInfo>
               &lt;/FareRedisplayMods>
            &lt;/PNRBFManagement_53>

         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXml>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>a08c6f20-efff-4834-8b43-d49d41c68c6c</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.RecordLoc</defaultValue>
      <description></description>
      <id>f150a826-c397-48e0-8414-7bfcc1f62c65</id>
      <masked>false</masked>
      <name>RecordLoc</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Provider</defaultValue>
      <description></description>
      <id>8e2a2437-7f52-4782-b61c-9400290e4174</id>
      <masked>false</masked>
      <name>Provider</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

println(&quot;LastName : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.LNameInfo.LName')))
println(&quot;FirstName : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.FNameInfo.FName')))
println(&quot;Seg Status : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.AirSeg.Status')))
println(&quot;Date : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.AirSeg.Dt')))
println(&quot;AirV : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.AirSeg.AirV')))
println(&quot;StartAirport : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.AirSeg.StartAirp')))
println(&quot;EndAirport : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.AirSeg.EndAirp')))
println(&quot;SSRType : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.NonProgramaticSSR.SSRCode')))
println(&quot;SSRText : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.NonProgramaticSSR.SSRText')))
println(&quot;PhoneNumber : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.PhoneInfo.Phone')))
println(&quot;Recordlocator : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.VndRecLocs.RecLocInfoAry.RecLocInfo.RecLoc')))
WS.verifyElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.GenPNRInfo.FareDataExistsInd', 'Y')
WS.verifyElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.PNRBFManagement_53.PNRBFRetrieve.GenPNRInfo.PNRBFTicketedInd', 'Y')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
