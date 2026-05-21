//! Back on Track — `{4}{B}` sorcery. "Return target creature or
//! Vehicle card from your graveyard to the battlefield. Create a 1/1
//! colorless Pilot creature token with [crew-helper text]." The Pilot
//! activated ability with crew/saddle rider isn't in the catalog;
//! we emit the reanimation and a vanilla Pilot token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Back on Track");
    let _pilot = reg.interner_mut().intern("Pilot");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature or Vehicle card from your graveyard to the battlefield. Create a 1/1 colorless Pilot creature token with \"This token saddles Mounts and crews Vehicles as though its power were 2 greater.\"".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Vehicle' card-type filter on graveyard target (TargetFilter
    // takes a single filter); we use creature(). Also GAP the Pilot
    // token's 'as though its power were 2 greater' rider — emit a
    // vanilla 1/1 Pilot.
    let mut effects = Vec::new();
    if let Some(target) = entry.targets.targets.first() {
        if let arcana_core::targets::TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
        }
    }
    let pilot = reg.interner().lookup("Pilot").expect("Pilot interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pilot);
    let token = TokenDefinition {
        name: pilot,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects
}
