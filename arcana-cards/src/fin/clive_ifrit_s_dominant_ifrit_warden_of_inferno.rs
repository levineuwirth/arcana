//! Clive, Ifrit's Dominant // Ifrit, Warden of Inferno — `{4}{R}{R}` Legendary Human Noble
//! Warrior 5/5 (front).
//!
//! Front face (Clive, Ifrit's Dominant):
//!   When Clive enters, you may discard your hand, then draw cards equal to your devotion to red.
//!   {4}{R}{R}, {T}: Exile Clive, then return it to the battlefield transformed under its owner's
//!   control. Activate only as a sorcery.
//!
//! Back face (Ifrit, Warden of Inferno) — Legendary Enchantment Creature — Saga Demon:
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I — Lunge — Ifrit fights up to one other target creature.
//!   II, III — Brimstone — Add {R}{R}{R}{R}. If Ifrit has three or more lore counters on it,
//!   exile it, then return it to the battlefield (front face up).
//!
//! GAP: "you may discard your hand" — discard-hand optional gate not expressible via
//!      OptionalPaymentKind (only Mana/Life are available); the draw-equal-to-devotion
//!      still fires unconditionally if the discard is not modeled.
//! GAP: Saga chapter trigger auto-advancement and chapter ability dispatch not in engine;
//!      back face modeled with a placeholder; Saga mechanics deferred.
//! GAP: Back-face-only Saga lore-counter triggers and fight/mana chapter abilities not
//!      auto-installed on transform.
//! GAP: "Exile Clive then return transformed" — exile-then-return-transformed is not a
//!      single engine effect; modeled as a direct Transform.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clive, Ifrit's Dominant");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(noble_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // Back face: Ifrit, Warden of Inferno — Legendary Enchantment Creature — Saga Demon
    let back_name = reg.interner_mut().intern("Ifrit, Warden of Inferno");
    let back_saga_sub = reg.interner_mut().intern("Saga");
    let back_demon_sub = reg.interner_mut().intern("Demon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_saga_sub);
    back_subtypes.0.insert(back_demon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: you may discard hand, then draw cards equal to devotion to red.
            // GAP: "you may discard your hand" optional cost not expressible; draws unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_devotion_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {4}{R}{R}, {T}: Exile Clive, then return transformed. Sorcery speed.
            // GAP: exile-then-return-transformed modeled as direct Transform.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}{R}, {T}: Exile Clive, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: front_transform,
            }),
        // GAP: back-face Saga chapter triggers (lore counter auto-advance, I fight, II-III mana
        //      and conditional return) not auto-installed on transform.
    )
}

/// ETB: draw cards equal to devotion to red.
/// GAP: "you may discard your hand" optional gate not modeled; draws unconditionally.
fn etb_devotion_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let devotion = script::devotion(state, trig.controller, ColorSet::red());
    if devotion == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: trig.controller, count: devotion }]
}

fn front_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
