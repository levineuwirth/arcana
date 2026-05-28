//! Chaos Harlequin — `{2}{R}{R}` 2/4 red Creature — Human.
//! {R}: Exile the top card of your library. If that card is a land card, this creature gets -4/-0 until end of turn.
//!   Otherwise, this creature gets +2/+0 until end of turn.
//! GAP: "exile top card; if land gets -4/-0, otherwise +2/+0" — conditional based on exiled card type not in catalog.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Harlequin");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}: Exile the top card. If land, -4/-0; otherwise +2/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: chaos_ability,
            }),
    )
}

fn chaos_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile top card; if land -4/-0, otherwise +2/+0" — conditional based on exiled card type not in catalog
    Vec::new()
}
