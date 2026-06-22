//! Angel of Glory's Rise — `{5}{W}{W}` 4/6 Creature — Angel.
//! Flying.
//! When this creature enters, exile all Zombies, then return all Human
//! creature cards from your graveyard to the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel of Glory's Rise");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_zombies_return_humans,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_zombies_return_humans(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Exile all Zombies (board-wide, any controller).
    let zombie_ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Zombie"),
        trig.controller,
    );

    // Return Human creature cards from YOUR graveyard. The demonstrated
    // graveyard return is non-targeted Reanimate over a filter; it returns a
    // single matching card per resolution. FIDELITY GAP: "all" Human creature
    // cards is approximated as the standard single-card Reanimate (no
    // documented graveyard-ids helper to enumerate "all" for a ForEach).
    let human_filter = match reg.interner().lookup("Human") {
        Some(sym) => ObjectFilter::creature().with_subtype_sym(sym),
        None => ObjectFilter::creature(),
    };

    vec![Effect::Sequence(vec![
        Effect::ForEach {
            targets: zombie_ids,
            effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
        },
        Effect::Reanimate {
            player: trig.controller,
            filter: human_filter,
            from_zone: Zone::Graveyard(trig.controller),
        },
    ])]
}
