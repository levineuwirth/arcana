//! Avatar of Growth — `{4}{G}{G}` 4/4 Elemental Avatar with Trample.
//!
//! This spell costs {1} less to cast for each opponent you have.
//! Trample
//! When this creature enters, each player searches their library for up to
//! two basic land cards, puts them onto the battlefield, then shuffles.
//!
//! Trample is wired. The ETB ramp is wired per player via two
//! `TutorToBattlefield` (each searching for a basic land) for every player.
//! GAP: the cost-reduction static ("costs {1} less for each opponent") has
//! no demonstrated cost-modification primitive on this shape.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avatar of Growth");
    let elemental = reg.interner_mut().intern("Elemental");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(avatar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        // GAP: "costs {1} less to cast for each opponent" — static cost
        // reduction, no demonstrated primitive on this shape.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_each_player_ramp,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_each_player_ramp(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let basic_land = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        // "up to two basic land cards" — two single-target searches.
        effects.push(Effect::TutorToBattlefield {
            player: p,
            filter: basic_land.clone(),
            tapped: false,
        });
        effects.push(Effect::TutorToBattlefield {
            player: p,
            filter: basic_land.clone(),
            tapped: false,
        });
    }
    vec![Effect::Sequence(effects)]
}
