//! The Sentry, Golden Guardian — `{3}{W}` 5/5 Legendary Human Hero with
//! Flying, Vigilance, and Indestructible.
//! "When The Sentry enters, target opponent creates The Void, a legendary 5/5
//! black Horror Villain creature token with flying, indestructible, and 'The
//! Void attacks each combat if able.'"

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Sentry, Golden Guardian");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(hero);
    // Pre-intern the token's subtype symbols.
    let _horror = reg.interner_mut().intern("Horror");
    let _villain = reg.interner_mut().intern("Villain");
    // PARTIAL: the created token's "legendary" supertype is not expressible
    // (TokenDefinition carries no supertypes). The 5/5 black flying/
    // indestructible Horror Villain body and its "attacks each combat if
    // able" self must-attack (a token ETB trigger, CR 508.1a) are created
    // below.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: create_the_void,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn create_the_void(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    let horror = reg.interner().lookup("Horror").unwrap_or_default();
    let villain = reg.interner().lookup("Villain").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    subtypes.0.insert(villain);
    vec![Effect::CreateToken {
        controller: *p,
        token: TokenDefinition {
            name: horror,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Indestructible],
            // "The Void attacks each combat if able." — the token's own ETB
            // trigger installs a self must-attack requirement (CR 508.1a).
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_void_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn install_void_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
