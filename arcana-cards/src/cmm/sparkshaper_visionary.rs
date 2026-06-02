//! Sparkshaper Visionary — `{2}{U}` 0/5 blue Human Wizard. "At the
//! beginning of combat on your turn, choose any number of target
//! planeswalkers you control. Until end of turn, they become 3/3 blue
//! Bird creatures with flying, hexproof, and 'Whenever this creature
//! deals combat damage to a player, scry 1.' (They're no longer
//! planeswalkers. Loyalty abilities can still be activated.)"
//!
//! GAP: Scryfall tagged "Scry" but the card has no standalone scry —
//! the scry lives inside the granted triggered ability, so no
//! card-level keyword is emitted.
//! GAP: the animation can't fully strip the planeswalker type (AddType
//! is additive) nor add the Bird subtype; the P/T set, color, evasion,
//! and the granted combat-damage scry trigger ARE applied per chosen
//! target.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sparkshaper Visionary");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: animate_planeswalkers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types(TypeLine::PLANESWALKER.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
        }),
    )
}

fn animate_planeswalkers(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        let TargetChoice::Object(id) = target else {
            continue;
        };
        let id = *id;
        effects.push(Effect::SetBasePT {
            target: id,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::SetColor {
            target: id,
            colors: ColorSet::blue(),
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Hexproof,
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::GrantTriggeredAbility {
            target: id,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: granted_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::EndOfTurn,
        });
    }
    effects
}

fn granted_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 1 }]
}
