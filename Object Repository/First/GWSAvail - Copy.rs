<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GWSAvail - Copy</name>
   <tag></tag>
   <elementGuidId>61c04562-ff3b-42ba-9cfb-f7dddb9df73e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic R1dTL0dBQ0dJVEVTVEhBUzpHaXRlc3Qx</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:web=&quot;http://webservices.galileo.com&quot; xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
&lt;soapenv:Header/>
&lt;soapenv:Body>
&lt;web:SubmitXml>
&lt;web:Profile>GWS_UAT_1V_2I3X_7A2115&lt;/web:Profile>
&lt;web:Request>
&lt;AirAvailability_19>
&lt;AirAvailMods>
  &lt;AirVPrefs>
            &lt;AirVAry>
                &lt;AirVInfo>
                    &lt;AirV>DL&lt;/AirV>
                &lt;/AirVInfo>
            &lt;/AirVAry>
        &lt;/AirVPrefs>
&lt;GenAvail>
&lt;NumSeats>2&lt;/NumSeats>
&lt;Class>Y&lt;/Class>
&lt;StartDt>20220101&lt;/StartDt>
&lt;StartPt>DEN&lt;/StartPt>
&lt;EndPt>ATL&lt;/EndPt>
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
  &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://apac.ut.travelport.com/B2BGateway/service/XMLSelect</soapServiceEndpoint>
   <soapServiceFunction>SubmitXmlOnSession</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Vendor1</defaultValue>
      <description></description>
      <id>6b003460-72d3-49cf-abe0-5a58307fc60e</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
