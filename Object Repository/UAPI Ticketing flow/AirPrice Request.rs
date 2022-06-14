<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirPrice Request</name>
   <tag></tag>
   <elementGuidId>436a414e-3547-460c-8b57-018e53e20012</elementGuidId>
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
      <value>Basic dUFQSTg4NDczMTI1ODUtMmJjMjRlMjQ6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soap:Body>
		&lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;${TargetBranch}&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:AirItinerary>
				${AirSegment}
               
			&lt;/air:AirItinerary>
			&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
			&lt;air:AirPricingCommand>
			&lt;/air:AirPricingCommand>
		&lt;/air:AirPriceReq>
	&lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.AirSegment</defaultValue>
      <description></description>
      <id>b94cf316-f7e4-41a6-a1d1-3added43fa79</id>
      <masked>false</masked>
      <name>AirSegment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TargetBranch</defaultValue>
      <description></description>
      <id>e8f351cd-2dbf-4c1a-ab90-ee0912183f69</id>
      <masked>false</masked>
      <name>TargetBranch</name>
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
