//! M'Odo, the Gnarled Oracle — `{B}{U}{G}` 0/3 Legendary Zombie Elf Wizard.
//! Eminence — `{X}, Discard a card:` target player reveals cards from the top
//! of their library until a creature card with mana value X or less is revealed;
//! put it onto the battlefield under your control, shuffle the rest away.
//! Activate only on the battlefield or in the command zone (Eminence).
//! GAP: the {X} mana cost itself now fans out, but the PAYLOAD is still not
//! faithfully expressible, so the whole ability is left gapped:
//!   - Effect::RevealUntil with found_dest Battlefield grants control to the
//!     REVEALED player (the library's owner), not to the activator — M'Odo puts
//!     the creature onto the battlefield under YOUR control, so an opponent-
//!     targeted reveal would seat it under the wrong controller.
//!   - "shuffle the rest into their library" has no exact DigRest (only
//!     bottom-random / graveyard).
//!   - Eminence command-zone activation (ActivationZone is Battlefield-only)
//!     is a separate GAP.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::Effect;
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("M'Odo, the Gnarled Oracle");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{U}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, Discard a card: Target player reveals cards from the top of their library until they reveal a creature card with converted mana cost X or less. Put that card onto the battlefield under your control, then that player shuffles the rest into their library. Activate this ability only if M'Odo, the Gnarled Oracle is on the battlefield or in the command zone.".into(),
                cost: ActivationCost {
                    discard_self: false,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: eminence_ability,
            }),
    )
}

fn eminence_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the {X} cost now fans out, but the payload is not faithfully
    // expressible — RevealUntil seats the found creature under the revealed
    // player's control (not the activator's), and "shuffle the rest into their
    // library" has no exact DigRest. Eminence command-zone activation is also
    // unmodeled. Left as a no-op rather than half-wired.
    Vec::new()
}
