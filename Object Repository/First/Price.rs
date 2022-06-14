<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Price</name>
   <tag></tag>
   <elementGuidId>803f32bd-5f1b-4258-b616-f7c5a53b3de1</elementGuidId>
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
      <value>Basic dHJhdmVscG9ydHN1cGVyYWRtaW46TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soap:Body>
      &lt;air:AirPriceReq TraceId=&quot;a153b6b0-a75b-4c34-b061-ad3685265772&quot; AuthorizedBy=&quot;UAPITesting&quot; TargetBranch=&quot;${GlobalVariable.TargetBranch}&quot; FareRuleType=&quot;none&quot; CheckFlightDetails=&quot;false&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
         &lt;air:AirItinerary>
        ${Segment}
         &lt;/air:AirItinerary>
         &lt;air:AirPricingModifiers FaresIndicator=&quot;AllFares&quot; ReturnServices=&quot;true&quot; InventoryRequestType=&quot;DirectAccess&quot;/>
         &lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;TuJJbrVu4hG1/9LlYIhwmw==&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
      &lt;!--&lt;com:SearchPassenger Code=&quot;CNN&quot; DOB=&quot;2015-09-09&quot; BookingTravelerRef=&quot;TuJJbrVu4hG1/9LlY1Ihwmw==&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>-->
         &lt;air:AirPricingCommand/>
      &lt;/air:AirPriceReq>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.EndPoint}/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.seg</defaultValue>
      <description></description>
      <id>b1eec7c1-32c0-4413-9d33-f74741353d7a</id>
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
