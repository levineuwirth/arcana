//! Alphinaud Leveilleur — `{3}{U}` 2/4 Legendary Elf Wizard.
//! * Partner with Alisaie Leveilleur (ETB: target player may tutor the partner
//!   into hand). Partner / Partner-with are not usable KeywordAbility variants,
//!   and the optional targeted-player tutor-by-name into THEIR own library has
//!   no expressible primitive — GAP'd (keyword line is Vigilance only).
//! * Vigilance (keyword).
//! * "Eukrasia — Whenever you cast your second spell each turn, draw a card." —
//!   wired as a SpellCast (you) trigger gated by an intervening-if that allows
//!   it only when exactly two spells have been cast by you this turn (the
//!   triggering spell is already counted, so count == 2 marks the second spell).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "Partner with Alisaie Leveilleur" — Partner / Partner-with is not a
// usable KeywordAbility variant, and the optional targeted-player tutor-by-name
// (into that player's own library) ETB has no expressible primitive.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alphinaud Leveilleur");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_spell_this_turn),
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_second_spell_this_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::spells_cast_this_turn(s, &ObjectFilter::new(), you) == 2
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
