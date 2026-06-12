//! Feywild Visitor — `{2}{U}` Legendary Enchantment — Background.
//! "Commander creatures you own have 'Whenever one or more nontoken
//! creatures you control deal combat damage to a player, you create a
//! 1/1 blue Faerie Dragon creature token with flying.'"
//!
//! Implementation: an ETB trigger installs a
//! `ContinuousEffect::filtered_grant_triggered` (Background recipe)
//! granting commander creatures a `DamageDealt` triggered ability
//! (nontoken creatures you control dealing combat damage to a player)
//! that mints a 1/1 blue Faerie Dragon with flying. "You own" is
//! approximated by `controlled_by(ControllerConstraint::You)`.
//! Fidelity note: "one or more … deal combat damage" is a BATCH
//! trigger (fires once per combat-damage event no matter how many
//! creatures connect); the engine's `DamageDealt` condition fires per
//! damaging creature, so multiple connecting creatures each mint a
//! token (a documented approximation).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feywild Visitor");
    let background = reg.interner_mut().intern("Background");
    let _faerie_dragon = reg.interner_mut().intern("Faerie Dragon");
    let _faerie = reg.interner_mut().intern("Faerie");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(background);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_grant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: grant commander creatures you control the
/// combat-damage Faerie Dragon trigger, anchored to this Background,
/// lasting while it remains on the battlefield.
fn etb_install_grant(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_grant_triggered(
            trig.source,
            ObjectFilter::creature()
                .commander_only()
                .controlled_by(ControllerConstraint::You),
            TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .nontoken()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: granted_make_faerie_dragon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Granted ability: "you create a 1/1 blue Faerie Dragon creature
/// token with flying."
fn granted_make_faerie_dragon(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg
        .interner()
        .lookup("Faerie Dragon")
        .expect("Faerie Dragon interned during register()");
    let faerie = reg
        .interner()
        .lookup("Faerie")
        .expect("Faerie interned during register()");
    let dragon = reg
        .interner()
        .lookup("Dragon")
        .expect("Dragon interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![arcana_core::effects::KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
