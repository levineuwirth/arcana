//! Dwarven Sea Clan — `{2}{R}` 1/1 red Dwarf.
//! "{T}: Choose target attacking or blocking creature whose controller controls an Island.
//! This creature deals 2 damage to that creature at end of combat. Activate only before
//! the end of combat step."
//! GAP: "attacking or blocking" + "controller controls an Island" TargetFilter constraint
//! not expressible; "deals damage at end of combat" (delayed damage) not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Sea Clan");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose target attacking or blocking creature whose controller controls an Island. This creature deals 2 damage to that creature at end of combat.".into(),
                cost: ActivationCost::tap_only(),
                // GAP: "attacking or blocking creature whose controller controls an Island"
                // not expressible; using target_creature() as approximation.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: delayed_damage_at_eoc,
            }),
    )
}

fn delayed_damage_at_eoc(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "deals 2 damage at end of combat" — delayed damage scheduled for end-of-combat
    // step is not expressible in the Effect catalog (DelayedAction does not support damage).
    Vec::new()
}
