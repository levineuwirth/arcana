//! Call of the Death-Dweller — `{2}{B}` sorcery. "Return up to two target
//! creature cards with total mana value 3 or less from your graveyard to the
//! battlefield. Put a deathtouch counter on either of them. Then put a
//! menace counter on either of them." No "total mana value" target
//! constraint; no deathtouch/menace counter kinds in the CounterKind catalog
//! (only PlusOnePlusOne). GAP the counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Call of the Death-Dweller");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target creature cards with total mana value 3 or less from your graveyard to the battlefield. Put a deathtouch counter on either of them. Then put a menace counter on either of them.".into(),
            // GAP: ObjectFilter expresses per-card max-cmc but not "total mana value 3 or less" across two targets.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(3),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    // GAP: CounterKind catalog only has PlusOnePlusOne; deathtouch/menace counters not available.
    out
}
