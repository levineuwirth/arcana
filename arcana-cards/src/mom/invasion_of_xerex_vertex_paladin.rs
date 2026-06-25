//! Invasion of Xerex // Vertex Paladin
//!
//! Battle — Siege (4 defense counters).
//! When this Siege enters, return up to one target creature to its owner's hand.
//! Back face: Vertex Paladin — Legendary Creature — Angel Knight.
//! Flying. Power and toughness each equal to the number of creatures you control.
//!   Wired as a Layer-7a self-CDA (`self_pt_from_match`, creatures you control)
//!   installed on `SelfTransforms{to_face:Some(1)}` with
//!   `Duration::WhileSourceOnBattlefield` — the front is a Battle that never
//!   returns, so the CDA lights up the moment it flips to the creature face.
//!   Back-face bones are `PtValue::Star`.
//!
//! GAP: Transform keyword not in implemented keyword set.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Xerex");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Vertex Paladin — Legendary Creature — Angel Knight
    let back_name = reg.interner_mut().intern("Vertex Paladin");
    let angel_sub = reg.interner_mut().intern("Angel");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(angel_sub);
    back_subtypes.0.insert(knight_sub);

    let back_chars = Characteristics {
        name: back_name,
        mana_cost: None,
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // */* — defined by the CDA (creatures you control) installed on transform.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 4,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_bounce,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // Vertex Paladin (back) CDA: P/T each equal to the number of
            // creatures you control. Installed when the Siege flips to the
            // creature face (the front never returns); lights up while on the
            // battlefield.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: install_back_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_transform_back(back_face)
    )
}

fn etb_bounce(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}

/// Layer 7a self-CDA: Vertex Paladin's P/T each equal to the number of
/// creatures you control.
fn install_back_cda(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
