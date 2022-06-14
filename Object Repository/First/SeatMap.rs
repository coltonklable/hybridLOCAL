<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SeatMap</name>
   <tag></tag>
   <elementGuidId>b3c8feac-db40-4ca6-bf39-8dce4a9cae99</elementGuidId>
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
      <value>Basic dUFQSTg1MDE0OTQ0ODctZDcxMTBhMmE6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:common=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;air:SeatMapReq AuthorizedBy=&quot;UAPITesting&quot; ReturnSeatPricing=&quot;true&quot; ReturnBrandingInfo=&quot;true&quot; TargetBranch=&quot;${GlobalVariable.TargetBranch}&quot; TraceId=&quot;1234&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;/>
          ${Segment}
         
      
   &lt;air:SearchTraveler Key=&quot;1&quot; Code=&quot;ADT&quot;>
            &lt;common:Name First=&quot;Rajiv&quot; Last=&quot;Sharma&quot;/>
         &lt;/air:SearchTraveler>
       &lt;!-- &lt;air:SearchTraveler Key=&quot;2&quot; Code=&quot;CNN&quot;>
            &lt;common:Name First=&quot;Karan&quot; Last=&quot;Sharma&quot;/>
         &lt;/air:SearchTraveler>-->
     &lt;air:HostReservation Carrier=&quot;${GlobalVariable.Carrier}&quot; CarrierLocatorCode=&quot;${GlobalVariable.CLC}&quot; ProviderCode=&quot;${GlobalVariable.ProviderCode}&quot; ProviderLocatorCode=&quot;${GlobalVariable.PNR}&quot; UniversalLocatorCode=&quot;${GlobalVariable.UR}&quot;/>
     &lt;/air:SeatMapReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.EndPoint}/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.RecordLocator</defaultValue>
      <description></description>
      <id>b1eec7c1-32c0-4413-9d33-f74741353d7a</id>
      <masked>false</masked>
      <name>AirRecordLocator</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.seg</defaultValue>
      <description></description>
      <id>1d0ceaaf-8099-45f8-a4be-6e89f54455c7</id>
      <masked>false</masked>
      <name>Segment</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementText(response, 'LowFareSearchRsp.AirSegmentList.AirSegment','')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
