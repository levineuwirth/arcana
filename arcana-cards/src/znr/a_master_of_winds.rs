//! A-Master of Winds — `{2}{U}{U}` 1/5 Sphinx Wizard with Flying.
//!
//! Oracle:
//! * Flying — keyword.
//! * "When Master of Winds enters, draw two cards, then discard a card."
//!   — ETB: draw two, then discard one (ordered; the discard posts a
//!   choice so the two steps are sequenced).
//! * "Whenever you cast an instant, sorcery, or Wizard spell, you may have
//!   Master of Winds's base power and toughness become 5/1 or 1/5 until
//!   end of turn." — a spell-cast trigger whose payload is a player choice
//!   between two base-P/T setups. No in-resolver modal choice primitive
//!   exists, so picking 5/1 vs 1/5 can't be expressed faithfully; the
//!   trigger condition is wired but its effect is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Master of Winds");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // "instant, sorcery, or Wizard spell" is an OR across a type set and a
    // subtype; a single ObjectFilter can't OR type-any with subtype-any
    // (chaining them ANDs the predicates). We match the instant/sorcery
    // portion faithfully; the Wizard-creature-spell branch is a partial.
    let spell_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let _ = wizard;

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_set_base_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: 2 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

fn cast_set_base_pt(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may have base P/T become 5/1 or 1/5" — a player choice
    // between two SetBasePT outcomes; no in-resolver modal choice
    // primitive to express the "5/1 or 1/5" selection.
    Vec::new()
}
