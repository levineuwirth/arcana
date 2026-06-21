//! Brigid, Hero of Kinsbaile — `{2}{W}{W}` 2/3 Legendary Kithkin Archer
//! with First strike.
//!
//! * First strike (keyword).
//! * `{T}: Brigid deals 2 damage to each attacking or blocking creature
//!   target player controls.`

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brigid, Hero of Kinsbaile");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(archer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Brigid deals 2 damage to each attacking or blocking creature target player controls.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: brigid_volley,
            }),
    )
}

fn brigid_volley(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each attacking or blocking creature target player controls" — no
    // documented combat-status (attacking/blocking) ObjectFilter refinement
    // to enumerate the affected creatures; dealing to all of that player's
    // creatures regardless of combat status would be materially wrong.
    Vec::new()
}
