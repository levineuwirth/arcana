//! Declaration in Stone exiles the targeted creature and its controller’s
//! same-name creatures, then investigates for each nontoken exiled.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Declaration in Stone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature and all other creatures its controller controls with the same name as that creature. That player investigates for each nontoken creature exiled this way.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return vec![];
    };
    let Some(target) = state.objects.get(*id) else {
        return vec![];
    };
    let controller = target.controller;
    let name = state.compute_characteristics(*id).unwrap().name;
    let mut count = 0;
    let mut effects = Vec::new();
    for obj in state.objects.objects_in_zone(Zone::Battlefield) {
        if obj.controller == controller
            && state.is_creature_now(obj.id)
            && state
                .compute_characteristics(obj.id)
                .is_some_and(|c| c.name == name)
        {
            if !obj.is_token {
                count += 1;
            }
            effects.push(Effect::ExilePermanent { target: obj.id });
        }
    }
    effects.push(Effect::CreateCommodityToken {
        controller,
        kind: CommodityToken::Clue,
        count,
    });
    effects
}
