<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TicketInfoDisplay</name>
   <tag></tag>
   <elementGuidId>358e790b-dd26-414a-964e-cf545b855ac0</elementGuidId>
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
            &lt;!--Use this request to display ticket info for a passenger from a from a PNR-->
            &lt;TicketInfoDisplay_20>
               &lt;PNRBFRetrieveMods>
                  &lt;PNRAddr>
                     &lt;RecLoc>${RecordLoc}&lt;/RecLoc>
                  &lt;/PNRAddr>
                  &lt;RequiredData>
                     &lt;PNRDataIDAry>
                        &lt;PNRDataID>TA&lt;/PNRDataID>
                     &lt;/PNRDataIDAry>
                  &lt;/RequiredData>
               &lt;/PNRBFRetrieveMods>
               &lt;ValidTicketInfoMods>
                  &lt;AssocPsgrs>
                     &lt;PsgrAry>
                        &lt;Psgr>
                           &lt;LNameNum>01&lt;/LNameNum>
                           &lt;PsgrNum>01&lt;/PsgrNum>
                           &lt;AbsNameNum>01&lt;/AbsNameNum>
                        &lt;/Psgr>
                     &lt;/PsgrAry>
                  &lt;/AssocPsgrs>
                  &lt;TicketTypeMod>
                     &lt;TicketType>E&lt;/TicketType>
                  &lt;/TicketTypeMod>
               &lt;/ValidTicketInfoMods>
            &lt;/TicketInfoDisplay_20>
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

println(&quot;BookingTravelerName : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.TicketInfoDisplay_20.ValidTicketInfo.ETicketNum.Name')))
println(&quot;FirstTkNum : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.TicketInfoDisplay_20.ValidTicketInfo.ETicketNum.FirstTkNum')))
println(&quot;Fare : &quot; + (WS.getElementText(response, 'SubmitXmlResponse.SubmitXmlResult.TicketInfoDisplay_20.ValidTicketInfo.ETicketNum.Fare')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
