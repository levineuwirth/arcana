//! Throne of the Grim Captain // The Grim Captain (transforming DFC, layout "transform").
//!
//! Front (Throne of the Grim Captain — Legendary Artifact, {2}):
//!   {T}: Mill two cards.
//!   Craft with a Dinosaur, a Merfolk, a Pirate, and a Vampire {4}.
//! Back (The Grim Captain — Legendary Creature — Skeleton Spirit Pirate):
//!   Menace, trample, lifelink, hexproof.
//!   Whenever The Grim Captain attacks, each opponent sacrifices a nonland permanent
//!     of their choice. Then you may put an exiled creature card used to craft
//!     The Grim Captain onto the battlefield under your control tapped and attacking.
//!
//! GAP: Craft (the front->back transform activated ability — exile this artifact plus
//!   four exact-subtype permanents/graveyard cards, then return transformed) is not
//!   expressible: ActivationCost has no multi-permanent / typed-exile / craft cost shape.
//!   The front->back transform is therefore unreachable via its printed cost.
//! GAP: the back attack-trigger's "put an exiled creature card used to craft this onto
//!   the battlefield tapped and attacking" rider depends on the unmodeled Craft-exile
//!   set; only the "each opponent sacrifices a nonland permanent" clause is emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Throne of the Grim Captain");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let spirit = reg.interner_mut().intern("Spirit");
    let pirate = reg.interner_mut().intern("Pirate");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("The Grim Captain");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(skeleton);
    back_subtypes.0.insert(spirit);
    back_subtypes.0.insert(pirate);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![
                KeywordAbility::Menace,
                KeywordAbility::Trample,
                KeywordAbility::Lifelink,
                KeywordAbility::Hexproof,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: {T}: Mill two cards.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Mill two cards.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: mill_two,
            })
            // GAP: "Craft with a Dinosaur, a Merfolk, a Pirate, and a Vampire {4}" —
            //   the craft transform-cost shape is not expressible. Ability omitted.
            // Back: Whenever The Grim Captain attacks, each opponent sacrifices a
            //   nonland permanent of their choice.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_each_opponent_sacrifices,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn mill_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill {
        player: ctx.controller,
        count: 2,
    }]
}

fn attack_each_opponent_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each opponent sacrifices a nonland permanent of their choice.
    let nonland = ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: nonland.clone(),
            count: 1,
        });
    }
    // GAP: "Then you may put an exiled creature card used to craft this onto the
    //   battlefield under your control tapped and attacking" — depends on the
    //   unmodeled Craft-exile set; omitted.
    effects
}
