//! Covert Cutpurse // Covetous Geist — `{2}{B}` Human Rogue 2/1 (front).
//!
//! Front face:
//! - When this creature enters, destroy target creature you don't control that
//!   was dealt damage this turn.
//! - Disturb {4}{B} — not in the usable keyword surface; omitted.
//!
//! Back face (Covetous Geist): Flying, deathtouch 2/1 Spirit Rogue.
//! - If Covetous Geist would be put into a graveyard from anywhere, exile it
//!   instead (replacement effect — GAP: graveyard-replacement not modeled).
//!
//! GAP: "dealt damage this turn" constraint on the destroy target is not
//! expressible with the available ObjectFilter API — the filter captures any
//! opponent creature (closest available approximation).
//! GAP: Disturb keyword not in the usable keyword surface; omitted.
//! GAP: Back-face graveyard-to-exile replacement effect not modeled.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Covert Cutpurse");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Covetous Geist — Flying, Deathtouch 2/1 Spirit Rogue
    let back_name = reg.interner_mut().intern("Covetous Geist");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let rogue_sub2 = reg.interner_mut().intern("Rogue");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    back_subtypes.0.insert(rogue_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
            ..Default::default()
        },
        spell_ability: None,
    };

    // ETB trigger: destroy target creature opponent controls (that was dealt
    // damage this turn — constraint not expressible, approximated as any
    // opponent creature).
    let target_req = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_req],
            }),
    )
}

fn etb_destroy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
