//! Alela, Cunning Conqueror — `{2}{U}{B}` 2/4 Legendary Faerie Warlock.
//! * Flying (keyword)
//! * "Whenever you cast your first spell during each opponent's turn, create a
//!   1/1 black Faerie Rogue creature token with flying." (triggered; PARTIAL)
//! * "Whenever one or more Faeries you control deal combat damage to a player,
//!   goad target creature that player controls." (triggered; PARTIAL)
//!
//! PARTIALs:
//! * Trigger 1 fires on each spell you cast (the closest condition); the
//!   "first ... during each opponent's turn" gate is not expressible (no
//!   per-opponent-turn first-spell condition), so the token may be created more
//!   often than printed. The token mint itself is faithful.
//! * Trigger 2's target restriction "that player controls" is the DAMAGED
//!   player's creatures (a dynamic controller dependent on the event);
//!   approximated as a creature an opponent controls. The "one or more Faeries"
//!   aggregation is collapsed to the per-source combat-damage trigger.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alela, Cunning Conqueror");
    let faerie = reg.interner_mut().intern("Faerie");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(warlock);
    // Pre-intern token subtypes so the resolver can rebuild them.
    let _rogue = reg.interner_mut().intern("Rogue");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let faerie_damage = TriggerCondition::DamageDealt {
        source_filter: script::subtype_filter(reg, "Faerie")
            .controlled_by(ControllerConstraint::You),
        target_filter: TargetFilter::Player,
        combat_only: true,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_faerie_rogue,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: faerie_damage,
                intervening_if: None,
                effect: goad_opponent_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn make_faerie_rogue(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let faerie = reg.interner().lookup("Faerie").unwrap_or_default();
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);
    let token = TokenDefinition {
        name: faerie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn goad_opponent_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Goad {
        target: *id,
        goader: trig.controller,
        duration: Duration::UntilYourNextTurn(trig.controller),
    }]
}
