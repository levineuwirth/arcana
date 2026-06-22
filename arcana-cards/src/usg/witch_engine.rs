//! Witch Engine — `{5}{B}` 4/4 Horror with Swampwalk.
//! "{T}: Add {B}{B}{B}{B}. Target opponent gains control of this creature.
//! (Activate only as an instant.)"
//!
//! Swampwalk maps to Landwalk("Swamp"). The activated ability adds four black
//! mana and gives control of this creature to a target opponent; it is not a
//! mana ability (it has a target and a non-mana effect) and is instant-speed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaUnit;
use arcana_core::types::ManaColor;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::mana::ManaCost;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witch Engine");
    let horror = reg.interner_mut().intern("Horror");
    let swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Landwalk(swamp)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add {B}{B}{B}{B}. Target opponent gains control of this creature.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: true,
            face_gate: None,
            effect: mana_and_give_control,
        }),
    )
}

fn mana_and_give_control(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source); 4],
        },
        Effect::ChangeControl {
            target: ctx.source,
            new_controller: *p,
        },
    ]
}
