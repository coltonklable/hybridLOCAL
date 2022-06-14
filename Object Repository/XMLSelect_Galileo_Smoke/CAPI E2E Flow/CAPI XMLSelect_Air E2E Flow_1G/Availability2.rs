<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Availability2</name>
   <tag></tag>
   <elementGuidId>787eca27-cfcc-432e-a478-bea744be224e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
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
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://webservices.galileo.com&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:SubmitXml>
         &lt;web:Profile>${Profile}&lt;/web:Profile>
         &lt;web:Request>
            &lt;AirAvailability_19>
               &lt;AirAvailMods>
                 &lt;AirVPrefInd>
                     &lt;AirVIncExcInd>I&lt;/AirVIncExcInd>
                     &lt;RelaxAirVPref>N&lt;/RelaxAirVPref>
                  &lt;/AirVPrefInd>
                  &lt;AirVPrefs>
            &lt;AirVAry>
                &lt;AirVInfo>
                    &lt;AirV>${Carrier}&lt;/AirV>
                &lt;/AirVInfo>
            &lt;/AirVAry>
        &lt;/AirVPrefs>
                  &lt;GenAvail>
                     &lt;NumSeats>2&lt;/NumSeats>
                     &lt;Class>Y&lt;/Class>
                     &lt;StartDt>${EndDate}&lt;/StartDt>
                     &lt;StartPt>${Destination}&lt;/StartPt>
                     &lt;EndPt>${Origin}&lt;/EndPt>
                     &lt;StartTm>1315&lt;/StartTm>
                     &lt;TmWndInd>D&lt;/TmWndInd>
                     &lt;StartTmWnd>0800&lt;/StartTmWnd>
                     &lt;EndTmWnd>2300&lt;/EndTmWnd>
                     &lt;JrnyTm/>
                     &lt;FltTypeInd/>
                     &lt;StartPtInd/>
                     &lt;EndPtInd/>
                     &lt;IgnoreTSPref/>
                  &lt;/GenAvail>
               &lt;/AirAvailMods>
            &lt;/AirAvailability_19>
         &lt;/web:Request>
         &lt;web:Filter>
            &lt;_/>
         &lt;/web:Filter>
      &lt;/web:SubmitXml>
      &lt;web:BeginSession/>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Profile</defaultValue>
      <description></description>
      <id>3ef50284-4531-4ff5-a29f-003b398060e1</id>
      <masked>false</masked>
      <name>Profile</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(4, 1)</defaultValue>
      <description></description>
      <id>2a26971f-d8de-4459-8201-ee3557887cba</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>6260ebbd-22ad-401f-a75f-6705ecf74b44</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>c6974240-9913-4111-8db0-6c9ff9d8bc31</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EndDate</defaultValue>
      <description></description>
      <id>bffe59d5-46a8-44bc-b53b-768f1313011d</id>
      <masked>false</masked>
      <name>EndDate</name>
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

println(&quot;AirV Values are &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt.AirV')))
println(&quot;FlightNumber &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].FltNum')))
println(&quot;StartDate &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartDt')))
println(&quot;Origin &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartAirp')))
println(&quot;Destination &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndAirp')))
println(&quot;StartTime &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].StartTm')))
println(&quot;EndTime &quot; + (WS.getElementPropertyValue(response, 'SubmitXmlResponse.SubmitXmlResult.AirAvailability_19.AirAvail.AvailFlt[0].EndTm')))
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
