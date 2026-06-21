//! Goblin Cratermaker — `{1}{R}` 2/2 Creature — Goblin Warrior.
//!
//! * `{1}, Sacrifice this creature: Choose one —`
//!   `• This creature deals 2 damage to target creature.`
//!   `• Destroy target colorless nonland permanent.`
//!   A modal ACTIVATED ability: `ActivatedAbilityDef` has no modal /
//!   mode-clause support (modal is only available on spell abilities),
//!   and each mode has a distinct target — so the choose-one effect is
//!   GAP'd while the cost (mana + sacrifice-self) is recorded.

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
    let name = reg.interner_mut().intern("Goblin Cratermaker");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice this creature: Choose one — This creature deals 2 damage to target creature; or Destroy target colorless nonland permanent.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: modal_choose_one,
            }),
    )
}

fn modal_choose_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "Choose one —" on an ACTIVATED ability is not
    // expressible (no mode dispatch / per-mode targets for activated
    // abilities; modal is spell-ability only).
    Vec::new()
}
