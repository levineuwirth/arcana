//! Sidequest: Hunt the Mark // Yiazmat, Ultimate Mark — `{3}{B}{B}` black transforming DFC (CR 712).
//!
//! Front face (Sidequest: Hunt the Mark): Enchantment.
//!   When this enchantment enters, destroy up to one target creature.
//!   At the beginning of your end step, if a creature died under an opponent's control this
//!   turn, create a Treasure token. Then if you control three or more Treasures, transform
//!   this enchantment.
//!
//! Back face (Yiazmat, Ultimate Mark): Legendary Creature — Dragon, 7/7 with Flying.
//!   {1}{B}, Sacrifice another creature or artifact: Yiazmat gains indestructible until end of
//!   turn. Tap it.
//!
//! # Notes / GAPs
//! - The end-step trigger's intervening-if ("if a creature died under an opponent's control
//!   this turn") is not expressible — no documented `conditions::`/`script::` predicate for
//!   "an opponent's creature died this turn." Left as `intervening_if: None` (fires each end
//!   step) and the Treasure is created; the "then if you control three or more Treasures,
//!   transform" rider is GAP'd (the documented `Effect::Conditional` `Condition` variants are
//!   not in scope, so the board-count gate + transform can't be authored faithfully).
//! - Back-face activated ability ({1}{B}, Sac another creature/artifact: indestructible EOT +
//!   tap) is not expressible with the documented triggered/transform/effect surface (no
//!   activated-ability shape). Back face emitted as a 7/7 flying Legendary Dragon.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::{Phase, Step};
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sidequest: Hunt the Mark");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };

    // Back face: Yiazmat, Ultimate Mark — black Legendary Creature — Dragon, 7/7 Flying.
    let back_name = reg.interner_mut().intern("Yiazmat, Ultimate Mark");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dragon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: ETB — destroy up to one target creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 0)
            // Front face: at the beginning of your end step, create a Treasure (gate GAP'd).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 0),
    )
}

fn etb_destroy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn end_step_treasure(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "if a creature died under an opponent's control this turn" gate is not
    // expressible (no documented predicate), and the "then if you control three or more
    // Treasures, transform this enchantment" rider has no documented Condition variant.
    // We create the Treasure faithfully and omit the gate + conditional transform.
    let _ = Phase::PreCombatMain;
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
