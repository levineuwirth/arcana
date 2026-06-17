//! Bloodsoaked Champion — `{B}` 2/1 Human Warrior.
//! "This creature can't block." — static (GAP: no can't-block static here).
//! "Raid — {1}{B}: Return this card from your graveyard to the battlefield.
//!  Activate only if you attacked this turn." — graveyard-activated; the
//!  return-to-battlefield is wired, but the "only if you attacked this turn"
//!  Raid timing restriction has no permitted condition helper (GAP).

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
    let name = reg.interner_mut().intern("Bloodsoaked Champion");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Raid" keyword has no usable KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "This creature can't block" static is not expressible here.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Raid — {1}{B}: Return this card from your graveyard to the battlefield. Activate only if you attacked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    // GAP: "Activate only if you attacked this turn" (Raid)
                    // has no permitted activation_condition helper.
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

fn return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
