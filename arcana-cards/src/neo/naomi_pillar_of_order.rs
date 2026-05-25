//! Naomi, Pillar of Order — `{3}{W}{B}` 4/4 legendary white-black Human Advisor.
//! "Whenever Naomi enters or attacks, if you control an artifact and
//! an enchantment, create a 2/2 white Samurai creature token with
//! vigilance."
//! GAP: the "if you control an artifact and an enchantment" intervening
//! condition is noted; intervening_if uses None per convention.
//! Two triggers (ETB + attacks) share the same effect fn.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Naomi, Pillar of Order");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);
    let _samurai = reg.interner_mut().intern("Samurai");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening_if — "if you control an artifact and an enchantment"
                intervening_if: None,
                effect: create_samurai_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: intervening_if — "if you control an artifact and an enchantment"
                intervening_if: None,
                effect: create_samurai_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_samurai_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let samurai = reg.interner().lookup("Samurai").expect("Samurai interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(samurai);
    let token = TokenDefinition {
        name: samurai,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
