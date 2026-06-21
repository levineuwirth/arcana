//! Master of Winds — `{2}{U}{U}` 1/4 Sphinx Wizard with Flying.
//!
//! Flying
//! When this creature enters, draw two cards, then discard a card.
//! Whenever you cast an instant, sorcery, or Wizard spell, you may have this
//! creature's base power and toughness become 4/1 or 1/4 until end of turn.
//!
//! Flying is a keyword. The ETB draw-two-then-discard is a `Sequence`. The
//! cast trigger fires on an instant or sorcery you cast (the "or Wizard
//! spell" disjunction across type+subtype can't be combined in one
//! `ObjectFilter` — that branch is a partial filter GAP); its effect lets
//! the player CHOOSE between 4/1 and 1/4 and is optional ("you may"), which
//! has no expressible two-mode-player-choice primitive, so the effect is
//! GAP'd.

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
    let name = reg.interner_mut().intern("Master of Winds");
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
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_two_discard_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // Partial filter: instant or sorcery (the "or Wizard spell"
                // disjunction can't be expressed in one ObjectFilter).
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
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

fn etb_draw_two_discard_one(
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
    // GAP: "you may have this creature's base P/T become 4/1 OR 1/4 until end
    // of turn" — optional, with a player choice between two SetBasePT values;
    // no two-mode-player-choice primitive.
    Vec::new()
}
