//! Bohn, Beguiling Balladeer — `{1}{U}{R}` Legendary 3/3 Creature — Human Bard.
//! "Each nonland card in your hand without foretell has foretell. Its foretell
//!  cost is equal to its mana cost reduced by {2}."
//! "Whenever you cast your second spell each turn, goad target creature an
//!  opponent controls."
//!
//! Decomposition:
//! - Keyword line: Goad — NOT in the usable keyword surface, so `keywords:
//!   vec![]`. (Goad here is the Scryfall keyword tag for the goad effect in the
//!   triggered ability below.)
//! - "Each nonland card in your hand … has foretell …" is a pure static
//!   continuous ability granting foretell — GAP (no expressible Effect).
//! - "Whenever you cast your second spell each turn, goad target creature an
//!   opponent controls." → a SpellCast(You) triggered ability gated by an
//!   intervening-if of "this is your second spell this turn".

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bohn, Beguiling Balladeer");
    let human = reg.interner_mut().intern("Human");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword `Goad` not in usable KeywordAbility surface (the goad
        // effect itself is modeled in the triggered ability below).
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Each nonland card in your hand without foretell has foretell"
    // — a continuous ability granting foretell, with no expressible Effect.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: Some(if_second_spell),
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

/// Intervening-if: fire only when this is the controller's SECOND spell this
/// turn (the cast that just went on the stack is counted, so count == 2).
fn if_second_spell(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::spells_cast_this_turn(s, &ObjectFilter::default(), you) == 2
}

fn goad_opponent_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Goad {
        target: *id,
        goader: trig.controller,
        duration: Duration::EndOfTurn,
    }]
}
