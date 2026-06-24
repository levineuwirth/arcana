//! Invasion of Mercadia // Kyren Flamewright (Battle — Siege, MOM transform).
//! Front (Invasion of Mercadia — {1}{R} Battle — Siege, 5 defense):
//!   When this Siege enters, you may discard a card. If you do, draw two cards.
//! Back (Kyren Flamewright — Creature — Goblin Spellshaper):
//!   {2}{R}, {T}, Discard a card: Create two 1/1 blue and red Elemental
//!     creature tokens. Creatures you control get +1/+0 and gain haste
//!     until end of turn.
//!
//! ETB "you may discard a card. If you do, draw two cards." is wired via
//! Effect::OptionalPayment { Discard(1) → draw 2 }.
//!
//! GAPs:
//! - When defeated, exile and cast the back face transformed (CR 310.11) is not
//!   auto-wired; the battle just goes to the graveyard. The back face is authored
//!   via with_transform_back so it is at least defined.
//! - Back face has no printed P/T in the spec; modeled as a 1/1 placeholder.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition, CardFace,
    CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::layers::Duration;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Mercadia");
    let siege = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege);

    // Pre-intern back-face + token subtypes for resolution.
    let _ = reg.interner_mut().intern("Goblin");
    let _ = reg.interner_mut().intern("Spellshaper");
    let _ = reg.interner_mut().intern("Elemental");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Kyren Flamewright");
    let goblin = reg.interner_mut().intern("Goblin");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(goblin);
    back_subtypes.0.insert(spellshaper);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            // Front (Siege) ETB: you may discard a card; if you do, draw two.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_may_discard_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (face 1): {2}{R}, {T}, Discard a card: create two Elementals,
            // creatures you control get +1/+0 and gain haste until end of turn.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, {T}, Discard a card: Create two 1/1 blue and red \
                    Elemental creature tokens. Creatures you control get +1/+0 and \
                    gain haste until end of turn."
                    .to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: flamewright_tokens,
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn etb_may_discard_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may discard a card. If you do, draw two cards."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 2,
        }),
        else_effect: None,
    }]
}

fn flamewright_tokens(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    let mut effects = vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: ctx.controller,
            token,
        },
    ];

    // Creatures you control get +1/+0 and gain haste until end of turn.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    for id in ids {
        effects.push(Effect::Pump {
            target: id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Haste],
        });
    }
    effects
}
