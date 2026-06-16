//! Halo-Charged Skaab — `{4}{U}` 4/4 blue Zombie.
//! "When this creature enters, each player mills two cards. Then you
//! may put an instant, sorcery, or battle card from your graveyard
//! on top of your library."
//! GAP: "battle" is not a supported TypeLine constant — approximating
//! as instant or sorcery only.
//! GAP: "you may put ... on top of your library" is PutOnTopOfLibrary
//! which is a targeted effect; approximating with Surveil as
//! placeholder since PutOnTopOfLibrary requires known object id.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Halo-Charged Skaab");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_all,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_mill_all(
    state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 2 })
        .collect();
    // GAP: "put instant/sorcery/battle from graveyard on top of library" — no targeted
    // graveyard-to-top-of-library effect in catalog; omitted
    effects
}
