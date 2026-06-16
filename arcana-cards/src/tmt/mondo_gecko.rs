//! Mondo Gecko — `{1}{U}{U}` 2/3 Legendary Lizard Mutant.
//!
//! Oracle:
//! * "{1}, Discard a card: Until end of turn, Mondo Gecko becomes the color
//!   of your choice and gains hexproof from that color." — an activated
//!   ability whose cost is `{1}` + `discard_other` (a card). The EFFECT is
//!   GAP'd: "becomes the color of your choice" (a player color choice that
//!   feeds the granted "hexproof from that color") and "protection/hexproof
//!   from a chosen color" are not expressible primitives.
//! * "Whenever Mondo Gecko deals combat damage to a player, draw a card for
//!   each color among permanents you control." — a combat-damage-to-a-player
//!   trigger; the amount ("for each COLOR among permanents you control") is
//!   GAP'd: there is no `script::` helper that counts distinct colors among
//!   permanents, and emitting a literal would be a materially wrong card.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mondo Gecko");
    let lizard = reg.interner_mut().intern("Lizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{1}, Discard a card: Until end of turn, Mondo Gecko becomes
            // the color of your choice and gains hexproof from that color."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Discard a card: Until end of turn, Mondo Gecko becomes the color of your choice and gains hexproof from that color.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_chosen_color,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_for_each_color,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn become_chosen_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "becomes the color of your choice and gains hexproof
    // from that color". The runtime color choice driving a "hexproof/
    // protection from that color" grant is not expressible (no chosen-color
    // SetColor + no protection/hexproof-from-color primitive).
    Vec::new()
}

fn draw_for_each_color(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — "draw a card for each color among permanents you
    // control". No script:: helper counts distinct colors among the
    // permanents you control, so the dynamic amount can't be computed; a
    // literal would be materially wrong.
    Vec::new()
}
