//! Foul Renewal — `{3}{B}` instant. "Return target creature card from your
//! graveyard to your hand. Target creature gets -X/-X until end of turn,
//! where X is the toughness of the card returned this way." Two targets:
//! a graveyard creature card and a battlefield creature. We use
//! `script::toughness_of` on the graveyard id (returns 0 if gone, but it's a
//! grave card, so just read the spec-time value).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Foul Renewal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return target creature card from your graveyard to your hand. Target creature gets -X/-X until end of turn, where X is the toughness of the card returned this way.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(grave)) = entry.targets.targets.first() else { return Vec::new(); };
    let x = script::toughness_of(state, *grave).max(0);
    let mut out = vec![Effect::ReturnFromGraveyardToHand { target: *grave }];
    if let Some(TargetChoice::Object(creature)) = entry.targets.targets.get(1) {
        out.push(Effect::Pump {
            target: *creature,
            power: -x,
            toughness: -x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    out
}
