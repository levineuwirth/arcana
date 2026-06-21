//! Disciple of the Ring — `{3}{U}{U}` 3/4 blue Human Wizard.
//!
//! Oracle:
//! * "{1}, Exile an instant or sorcery card from your graveyard: Choose
//!   one —
//!     • Counter target noncreature spell unless its controller pays {2}.
//!     • This creature gets +1/+1 until end of turn.
//!     • Tap target creature.
//!     • Untap target creature."
//!
//! Modeled as a single activated ability. The `{1}` mana portion of the
//! cost is wired; the "Exile an instant or sorcery card from your
//! graveyard" portion is GAP'd (there is no exile-from-graveyard cost
//! field — ActivationCost has no such field, and OptionalPaymentKind is
//! Mana/Life only). The "Choose one —" modal body is GAP'd: modal
//! mode-selection (dispatch_modal_effect / with_mode_effects) is available
//! only on spell abilities, not on activated abilities, so the four modes
//! cannot be posted as a player choice here.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disciple of the Ring");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Exile an instant or sorcery card from your graveyard: Choose one — Counter target noncreature spell unless its controller pays {2}; or this creature gets +1/+1 until end of turn; or tap target creature; or untap target creature.".into(),
                // GAP (cost): the "Exile an instant or sorcery card from your
                // graveyard" portion of the cost is not expressible (no
                // exile-from-graveyard ActivationCost field); only the {1}
                // mana portion is wired.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: choose_one_gap,
            }),
    )
}

fn choose_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Choose one —" four-mode modal body. Modal mode-selection is
    // spell-ability only; not expressible on an activated ability.
    Vec::new()
}
