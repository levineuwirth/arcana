//! Hound Tamer // Untamed Pup (transforming DFC, layout "transform")
//!
//! Front face: Hound Tamer — {2}{G} Creature — Human Werewolf, 3/3.
//!   Trample
//!   {3}{G}: Put a +1/+1 counter on target creature.
//!   Daybound.
//! Back face: Untamed Pup — Creature — Werewolf, 3/3 (same printed stats).
//!   Trample
//!   Other Wolves and Werewolves you control have trample.
//!   {3}{G}: Put a +1/+1 counter on target creature.
//!   Nightbound.
//!
//! GAP: Daybound / Nightbound automatic day/night transform is not expressible —
//!   there is no exposed day/night-change trigger condition in the demonstrated API.
//! GAP: back-face static "Other Wolves and Werewolves you control have trample" is a
//!   continuous keyword-granting anthem; no static-ability / continuous-grant primitive
//!   is in the demonstrated API. Trample is printed on both faces of this card itself.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hound Tamer");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Untamed Pup");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {3}{G}: Put a +1/+1 counter on target creature. (Both faces.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: Put a +1/+1 counter on target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_counter,
            }),
    )
}

fn put_counter(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
