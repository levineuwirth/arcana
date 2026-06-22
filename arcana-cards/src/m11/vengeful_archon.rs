//! Vengeful Archon — `{4}{W}{W}{W}` 7/7 Archon with Flying.
//! `{X}: Prevent the next X damage that would be dealt to you this turn. If
//! damage is prevented this way, this creature deals that much damage to
//! target player or planeswalker.`
//!
//! The keyword line (Flying) is a base characteristic. The activated ability's
//! cost is a variable {X} mana payment, and its effect is a damage-prevention
//! shield that conditionally reflects the prevented amount back at a target —
//! a coupled prevent-and-reflect with a dynamic X that is not expressible with
//! the demonstrated `Effect`/`ActivationCost` surface, so it is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Archon");
    let archon = reg.interner_mut().intern("Archon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(archon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}: Prevent the next X damage that would be dealt to you this turn. If damage is prevented this way, this creature deals that much damage to target player or planeswalker.".into(),
            cost: ActivationCost::default(),
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gap_prevent_and_reflect,
        }),
    )
}

fn gap_prevent_and_reflect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: {X}: prevent the next X damage to you this turn, then if damage was
    // prevented this way deal that much damage to target — a coupled
    // dynamic-X prevent-then-reflect (the {X} cost feeds both the prevention
    // amount and the reflected damage). Not expressible: ActivationCost has no
    // {X} mana field and PreventDamage cannot feed its prevented amount into a
    // follow-up DealDamage.
    Vec::new()
}
