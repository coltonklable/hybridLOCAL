<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AMOA_1</name>
   <tag></tag>
   <elementGuidId>56a0be54-bc03-42f1-8d41-c90ecc43bd20</elementGuidId>
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
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soapenv:Body>
      &lt;air:AirMerchandisingOfferAvailabilityReq TargetBranch=&quot;P7113929&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;uAPI&quot;/>
         &lt;air:HostReservation Carrier=&quot;${GlobalVariable.Carrier}&quot; CarrierLocatorCode=&quot;${GlobalVariable.CLC}&quot; ProviderCode=&quot;${GlobalVariable.ProviderCode}&quot; ProviderLocatorCode=&quot;${GlobalVariable.PNR}&quot; UniversalLocatorCode=&quot;${GlobalVariable.UR}&quot;/>
      &lt;/air:AirMerchandisingOfferAvailabilityReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
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
