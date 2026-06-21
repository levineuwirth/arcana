//! Wakestone Gargoyle — `{3}{W}` 3/4 Gargoyle with Defender and Flying.
//! "{1}{W}: Creatures you control with defender can attack this turn as
//!  though they didn't have defender."
//!
//! Defender and Flying are base keywords. The activated ability granting
//! "can attack this turn as though they didn't have defender" has no
//! expressible Effect (no attack-permission-despite-Defender primitive),
//! so the ability's effect body is GAP'd while the cost shell is emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wakestone Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{W}: Creatures you control with defender can attack this turn as though they didn't have defender.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: attack_despite_defender,
            }),
    )
}

fn attack_despite_defender(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "creatures you control with defender can attack this turn as
    // though they didn't have defender" — no Effect expresses attack
    // permission despite Defender.
    Vec::new()
}
