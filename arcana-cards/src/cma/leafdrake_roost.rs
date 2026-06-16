//! Leafdrake Roost — `{3}{G}{U}` enchantment — Aura.
//! "Enchant land. Enchanted land has \"{G}{U}, {T}: Create a 2/2 green and
//!  blue Drake creature token with flying.\""
//!
//! Enchant-land Aura. The granted host activated ability is an ETB-installed
//! `attached_activated` continuous effect (the Squirrel Nest pattern); its
//! {G}{U},{T} cost and token-creation effect run against the host land.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leafdrake Roost");
    let aura = reg.interner_mut().intern("Aura");
    let _drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_drake_ability,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_drake_ability(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let ability = ActivatedAbilityDef {
        text: "{G}{U}, {T}: Create a 2/2 green and blue Drake creature token with flying.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{G}{U}").expect("valid cost"),
            tap: true,
            ..Default::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: create_drake,
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ability,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn create_drake(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let drake = match reg.interner().lookup("Drake") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    let token = TokenDefinition {
        name: drake,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
