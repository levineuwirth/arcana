//! Ashling, Flame Dancer — `{2}{R}{R}` 4/4 Legendary Elemental Shaman.
//! You don't lose unspent red mana as steps and phases end.
//! Magecraft — Whenever you cast or copy an instant or sorcery spell,
//! discard a card, then draw a card. If this is the second time this
//! ability has resolved this turn, Ashling deals 2 damage to each opponent
//! and each creature they control. If it's the third time, add {R}{R}{R}{R}.
//!
//! Magecraft is not a usable keyword variant; the trigger is wired as
//! `SpellCast { instant-or-sorcery, You }`. The escalating rider keys off
//! `script::spells_cast_this_turn` (close proxy for "times this ability has
//! resolved this turn" — the "or copy" half and exact resolution-count are
//! a fidelity GAP). The "don't lose unspent red mana" static is GAP'd (no
//! mana-burn / mana-emptying primitive).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashling, Flame Dancer");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);

    // GAP: "You don't lose unspent red mana as steps and phases end." — no
    // mana-retention static primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: magecraft,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn magecraft(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ])];

    // The triggering cast is already counted, so "second time" == 2 spells
    // cast this turn (close proxy for "times this ability resolved").
    let times = script::spells_cast_this_turn(
        state,
        &ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        trig.controller,
    );

    if times == 2 {
        for opp in script::opponents(state, trig.controller) {
            out.push(Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Player(opp),
                amount: 2,
            });
            // Creatures controlled by this opponent: take their perspective
            // (controlled_by(You) relative to `opp`).
            let creatures = script::ids_matching(
                state,
                &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                opp,
            );
            for id in creatures {
                out.push(Effect::DealDamage {
                    source: trig.source,
                    target: DamageTarget::Object(id),
                    amount: 2,
                });
            }
        }
    } else if times == 3 {
        out.push(Effect::AddMana {
            player: trig.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); 4],
        });
    }

    out
}
