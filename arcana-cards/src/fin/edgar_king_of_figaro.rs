//! Edgar, King of Figaro — `{4}{U}{U}` 4/5 Legendary Human Artificer Noble.
//! When Edgar enters, draw a card for each artifact you control.
//! Two-Headed Coin — The first time you flip one or more coins each turn,
//! those coins come up heads and you win those flips. — GAP: coin-flip
//! replacement static not expressible.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Edgar, King of Figaro");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    // GAP: "Two-Headed Coin" — a coin-flip replacement/static ("the first time
    // you flip one or more coins each turn, those coins come up heads and you
    // win those flips") is not expressible as a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draw_per_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draw_per_artifact(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
        trig.controller,
    );
    vec![Effect::DrawCards {
        player: trig.controller,
        count: n,
    }]
}
